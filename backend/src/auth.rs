const FIFTHTRY_SUPPORT_ORG_SLUG: &str = "fifthtry-support";

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Read logged in user data from request
/// Return error if user is not logged in
pub struct RequiredUser {
    #[serde(skip)]
    pub user_id: i64,
    pub username: String,
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
            name: fud.name,
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

pub(crate) fn is_fifthtry_staff_org_member(
    conn: &mut ft_sdk::Connection,
    user_id: i64,
) -> Result<bool, ft_sdk::Error> {
    use diesel::prelude::*;

    let count: i64 = ft_org_member::table
        .filter(ft_org_member::user_id.eq(user_id))
        .filter(ft_org::slug.eq(FIFTHTRY_SUPPORT_ORG_SLUG))
        .inner_join(ft_org::table)
        .count()
        .get_result(conn)?;

    Ok(count > 0)
}

// Copied from ft2::schema
diesel::table! {
    ft_org (id) {
        id -> BigInt,
        name -> Text,
        slug -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        owner_id -> BigInt,
        plan_id -> Nullable<BigInt>,
    }
}

diesel::table! {
    ft_org_member (id) {
        id -> BigInt,
        role -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        org_id -> BigInt,
        user_id -> BigInt,
    }
}

diesel::joinable!(ft_org_member -> ft_org (org_id));
diesel::allow_tables_to_appear_in_same_query!(ft_org, ft_org_member,);
