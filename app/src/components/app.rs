use crate::components::photo_grid::PhotoGrid;
use crate::error_template::AppError;
use crate::error_template::ErrorTemplate;
use crate::models::user::Album;
use leptos::create_resource;
use leptos::create_server_action;
use leptos::create_signal;
use leptos::server;
use leptos::Action;
use leptos::Errors;
use leptos::IntoView;
use leptos::ServerFnError;
use leptos::Transition;
use leptos::*;
use leptos::{component, view, SignalGet};
use leptos_meta::provide_meta_context;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::ActionForm;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;
use tracing::error;

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use super::*;
    use axum_session_auth::SessionSqlitePool;
    use leptos::{use_context, ServerFnError};
    use sqlx::SqlitePool;

    type AuthSession = axum_session_auth::AuthSession<Album, i64, SessionSqlitePool, SqlitePool>;
    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }

    pub fn pool() -> Result<SqlitePool, ServerFnError> {
        use_context::<SqlitePool>()
            .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }
}

#[server]
pub async fn get_user() -> Result<Option<Album>, ServerFnError> {
    use self::ssr::auth;
    let auth = auth()?;

    Ok(auth.current_user)
}

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();

    let user = create_resource(
        move || (login.version().get(), logout.version().get()),
        move |_| get_user(),
    );

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/photo-grid.css"/>
        <Title text="Photos"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <header>
            <Transition fallback=move || {
                view! { <span>"Loading..."</span> }
            }>
                {move || {
                    user.get()
                        .map(|user| match user {
                            Err(e) => {
                                error!("Login error: {}", e);
                                view! {
                                    // no content -> ErrorTemplate will be shown anyway
                                }
                                .into_view()
                            }

                            Ok(None) => {
                                view! {
                                    <Login action=login/>
                                }
                                .into_view()
                            }

                            Ok(Some(user)) => {
                                // TODO: get images for album
                                let images: Vec<String> = fetch_album(user.username).await.unwrap_or_else(|a| a);
                                // TODO: get image binary for each image
                                let image_data = fetch_image_data().await.unwrap_or_else(|_| String::new());

                                let (album, _set_album) = create_signal(user.username);

                                view! {
                                    <Logout action=logout album=album />
                                    <PhotoGrid images=images />
                                }
                                .into_view()
                            }
                        })
                }}
            </Transition>
            </header>

               <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="login" view=move || view! { <Login action=login/> }/>
                </Routes>
            </main>
        </Router>
    }
}
#[component]
pub fn Home() -> impl IntoView {}

#[server]
pub async fn fetch_album(album_id: String) -> Result<Vec<String>, ServerFnError> {
    let response = reqwest::get(format!("http://localhost:3000/api/album/{}", album_id))
        .await?
        .json::<Vec<String>>()
        .await?;

    Ok(response)
}

#[server]
pub async fn fetch_image_data(
    album_id: String,
    image_id: String,
) -> Result<Vec<String>, ServerFnError> {
    let response = reqwest::get(format!(
        "http://localhost:3000/api/album/{}/{}?w=300",
        album_id, image_id
    ))
    .await?
    .json::<Vec<String>>()
    .await?;

    Ok(response)
}

#[server(Login, "/api")]
pub async fn login(albumcode: String, passcode: String) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let pool = pool()?;
    let auth = auth()?;

    let user = Album::validate_credentials(albumcode, passcode, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("Albumcode not found!"))?;

    auth.login_user(user.id);
    leptos_axum::redirect("/");

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use self::ssr::*;

    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div class="center-screen">
            <ActionForm action=action>
                <label>
                    <input
                        type="text"
                        placeholder="Albumcode"
                        maxlength="32"
                        name="albumcode"
                        class="auth-input"
                    />
                </label>
                <br/>
                <label>
                    <input type="passcode" placeholder="Passcode" name="passcode" class="auth-input"/>
                </label>
                <br/>
                <input type="hidden" name="id"/>
                <input type="submit" class="button" value="Open album"/>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn Logout(
    action: Action<Logout, Result<(), ServerFnError>>,
    album: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <div class="selected_album">{format!("Selected album: {}", album.get_untracked())} </div>
            <ActionForm action=action>
            <button type="submit" class="button">
                "Change album"
            </button>
        </ActionForm>

    }
}
