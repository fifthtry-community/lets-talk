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
    pub(crate) fn is_allowed_to_create_meeting(&self, conn: &mut ft_sdk::Connection) -> bool {
        use diesel::prelude::*;

        let config = match talk_config::table
            .select(TalkConfig::as_select())
            .order_by(talk_config::id.desc())
            .first(conn)
        {
            Err(e) => {
                ft_sdk::println!("Error getting talk config: {:?}", e);
                return false;
            }
            Ok(v) => v,
        };

        config
            .allowed_usernames
            .split(',')
            .map(|v| v.trim())
            .chain(config.allowed_emails.split(',').map(|v| v.trim()))
            .chain(config.allowed_email_domains.split(',').map(|v| v.trim()))
            .any(|v| {
                if v.is_empty() {
                    return false;
                }

                if v.contains('@') {
                    // check email or email domain
                    let m = self.email == v || self.email.ends_with(v);
                    // if the email matches, check if the account is verified (verification is done
                    // by clicking on a link that is sent to user's email)
                    if config.require_verification {
                        return m && self.email_is_verified;
                    }

                    return m;
                }

                if config.require_verification {
                    return self.username == v && self.email_is_verified;
                }
                self.username == v
            })
    }
}

#[derive(Debug, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = talk_config)]
struct TalkConfig {
    allowed_usernames: String,
    allowed_emails: String,
    allowed_email_domains: String,
    require_verification: bool,
}

diesel::table! {
    talk_config (id) {
        id -> BigInt,
        // comma separated list of allowed usernames, emails, email domains
        // eg.:
        // "allowed_usernames": "alice,bob",
        // "allowed_emails": "alice@acmen.com",
        // "allowed_email_domains": "acme.com" // only acme.com emails are allowed
        allowed_usernames -> Text,
        allowed_emails -> Text,
        allowed_email_domains -> Text,
        require_verification -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(talk_config,);
