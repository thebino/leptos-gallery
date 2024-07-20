use leptos::{server, ServerFnError, use_context};
use sqlx::SqlitePool;
use crate::models::user::*;

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    pub use bcrypt::{hash, verify, DEFAULT_COST};

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))?;
    let auth = auth()?;

    let (user, UserPasshash(expected_passhash)) =
        User::get_from_username_with_passhash(username, &pool)
            .await
            .ok_or_else(|| ServerFnError::new("User does not exist."))?;

    match verify(password, &expected_passhash)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}
