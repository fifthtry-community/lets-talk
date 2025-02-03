const FIFTHTRY_SUPPORT_ORG_SLUG: &str = "fifthtry-support";

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
    /// It uses the following environment variables to dictate if the user is allowed to create a
    /// meeting:
    /// `LETS_TALK_ALLOWED_EMAIL_DOMAINS` (comma separated list of email domains)
    /// `LETS_TALK_REQUIRE_VERIFICATION` (true/false)
    ///
    /// If `LETS_TALK_REQUIRE_VERIFICATION` is set to true, then the user account must be verified
    pub(crate) fn is_special(&self, conn: &mut ft_sdk::Connection, config: &crate::Config) -> bool {
        use diesel::prelude::*;

        if config.allowed_emails.is_empty() {
            ft_sdk::println!(
                "`allowed-emails` list is empty. No one is allowed to create meetings"
            );
            return false;
        }

        config.allowed_emails.iter().map(|v| v.trim()).any(|v| {
            if v.is_empty() {
                return false;
            }

            // check email domain
            let email_matches = self.email.ends_with(v);
            // if the email matches, check if the account is verified (verification is done
            // by clicking on a link that is sent to user's email)
            if config.require_verification {
                return email_matches && self.email_is_verified;
            }

            return email_matches;
        })
    }
}
