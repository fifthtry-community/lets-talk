#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Read logged in user data from request
/// Return error if user is not logged in
pub struct RequiredUser {
    #[serde(skip)]
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub email_is_verified: bool,
    pub name: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Read logged in user data from request
/// Return default if user is not logged in: is_logged_in = false
pub struct OptionalUser {
    #[serde(skip)]
    pub user_id: i64,
    pub username: String,
    pub name: String,
    pub is_logged_in: bool,
}

impl ft_sdk::FromRequest for RequiredUser {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let conn = &mut ft_sdk::FromRequest::from_request(req)?;
        let cookie = ft_sdk::Cookie::<{ ft_sdk::auth::SESSION_KEY }>::from_request(req)?;

        let fud = match ft_sdk::auth::ud(cookie, conn)? {
            Some(v) => v,
            None => {
                return Err(ft_sdk::SpecialError::Single(
                    "auth".to_string(),
                    "User not logged in".to_string(),
                )
                .into());
            }
        };

        Ok(Self {
            user_id: fud.id,
            username: fud.identity,
            email: fud.email,
            name: fud.name,
            email_is_verified: fud.verified_email,
        })
    }
}

impl ft_sdk::FromRequest for OptionalUser {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let conn = &mut ft_sdk::FromRequest::from_request(req)?;
        let cookie = ft_sdk::Cookie::<{ ft_sdk::auth::SESSION_KEY }>::from_request(req)?;

        let fud = match ft_sdk::auth::ud(cookie, conn)? {
            Some(v) => v,
            None => return Ok(Default::default()),
        };

        Ok(Self {
            user_id: fud.id,
            username: fud.identity,
            name: fud.name,
            is_logged_in: true,
        })
    }
}

impl RequiredUser {
    /// Special users are the ones who have access to meeting data and are able to create new
    /// meetings.
    /// It uses the following settings to dictate if the user is allowed to create a meeting:
    /// `allowed-emails`       (comma separated list of email domains)
    /// `require-verification` (true/false)
    ///
    /// If `require-verification` is set to true, then the user account must be verified
    pub(crate) fn is_special(
        &self,
        config: &crate::Config,
        host: &ft_sdk::Host,
    ) -> Result<(), AuthorizationError> {
        let allowed_emails = get_allowed_emails_or_host(config.allowed_emails.clone(), host)?;

        let email_matches = allowed_emails
            .split(',')
            .map(|v| v.trim())
            .any(|v| !v.is_empty() && self.email.ends_with(v));

        if !email_matches {
            return Err(AuthorizationError::Unauthorized);
        }

        if config.require_verification && !self.email_is_verified {
            return Err(AuthorizationError::RequiresVerification);
        }

        Ok(())
    }
}

/// Get the allowed emails from the `who-can-create-meetings` ftd variable or the request host if
/// it's empty
/// Returns [AuthorizationError::EmptyWhitelist] if both are empty
fn get_allowed_emails_or_host(
    allowed_emails: String,
    host: &ft_sdk::Host,
) -> Result<String, AuthorizationError> {
    if !allowed_emails.is_empty() {
        return Ok(allowed_emails);
    }

    let domain = match host
        .without_port()
        .split('.')
        .collect::<Vec<&str>>()
        .as_slice()
    {
        // foo.bar.gov.in -> bar.gov.in
        [.., subdomain, domain, ext1, ext2] => Some(format!("{}.{}.{}", domain, ext1, ext2)),
        // meet.fifthtry.com -> fifthtry.com
        [.., subdomain, domain, ext] => Some(format!("{}.{}", domain, ext)),
        // fifthtry.com/talk/
        [host, ext] => Some(format!("{}.{}", host, ext)),
        _ => None,
    };

    match domain {
        Some(v) => Ok(v),
        None => Err(AuthorizationError::EmptyWhitelist),
    }
}

#[derive(Debug, PartialEq)]
pub enum AuthorizationError {
    /// The logged in user is not in `who-can-create-meetings` ftd variable
    Unauthorized,
    /// The `who-can-create-meetings` ftd variable is empty
    EmptyWhitelist,
    /// The user needs to verify their email
    RequiresVerification,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_allowed_emails_or_host() {
        let host = ft_sdk::Host("meet.fifthtry.com".to_string());
        let allowed_emails = "fifthtry.com".to_string();
        assert_eq!(
            get_allowed_emails_or_host(allowed_emails, &host).unwrap(),
            "fifthtry.com"
        );

        let host = ft_sdk::Host("meet.fifthtry.com".to_string());
        let allowed_emails = "".to_string();
        assert_eq!(
            get_allowed_emails_or_host(allowed_emails, &host).unwrap(),
            "fifthtry.com"
        );

        let host = ft_sdk::Host("fifthtry.com".to_string());
        let allowed_emails = "".to_string();
        assert_eq!(
            get_allowed_emails_or_host(allowed_emails, &host).unwrap(),
            "fifthtry.com"
        );

        let host = ft_sdk::Host("fifthtry.com".to_string());
        let allowed_emails = "capybara.com".to_string();
        assert_eq!(
            get_allowed_emails_or_host(allowed_emails, &host).unwrap(),
            "capybara.com"
        );

        let host = ft_sdk::Host("foo.bar.gov.in".to_string());
        let allowed_emails = "".to_string();
        assert_eq!(
            get_allowed_emails_or_host(allowed_emails, &host).unwrap(),
            "bar.gov.in"
        );
    }

    #[test]
    fn test_is_special() {
        // unverfied user with require_verification = false
        let user = user_not_verified("test@mail.com");
        let config = config_without_verification("mail.com");
        let host = ft_sdk::Host("meet.fifthtry.com".to_string());
        assert!(user.is_special(&config, &host).is_ok());

        // unverified user with require_verification = true
        let user = user_not_verified("test@mail.com");
        let config = config_with_verification("mail.com");
        assert_eq!(
            user.is_special(&config, &host),
            Err(AuthorizationError::RequiresVerification)
        );

        // verified user with require_verification = true
        let user = user_verified("test@mail.com");
        let config = config_with_verification("mail.com");
        assert!(user.is_special(&config, &host).is_ok());

        // verified user with require_verification = false
        let user = user_verified("test@mail.com");
        let config = config_without_verification("mail.com");
        assert!(user.is_special(&config, &host).is_ok());

        // verified user with empty allowed list but host matches
        let user = user_verified("siddhant@fifthtry.com");
        let config = config_without_verification("");
        let host = ft_sdk::Host("fifthtry.com".to_string());
        assert_eq!(user.is_special(&config, &host), Ok(()));
        // verified user with empty allowed list and sub host
        let host = ft_sdk::Host("talk.fifthtry.com".to_string());
        let config = config_with_verification("");
        assert!(user.is_special(&config, &host).is_ok());

        // verified user with allowed list different from host
        let host = ft_sdk::Host("fifthtry.com".to_string());
        let config = config_with_verification("capybara.com");
        let user = user_verified("siddhant@fifthtry.com");
        assert!(user.is_special(&config, &host).is_err());
        let user = user_verified("siddhant@capybara.com");
        assert!(user.is_special(&config, &host).is_ok());

        fn config_with_verification(emails: &str) -> crate::Config {
            crate::Config {
                allowed_emails: emails.to_string(),
                require_verification: true,
                ..Default::default()
            }
        }
        fn config_without_verification(emails: &str) -> crate::Config {
            crate::Config {
                allowed_emails: emails.to_string(),
                require_verification: false,
                ..Default::default()
            }
        }
        fn user_verified(email: &str) -> RequiredUser {
            RequiredUser {
                email: email.to_string(),
                email_is_verified: true,
                ..Default::default()
            }
        }
        fn user_not_verified(email: &str) -> RequiredUser {
            RequiredUser {
                email: email.to_string(),
                email_is_verified: false,
                ..Default::default()
            }
        }
    }
}
