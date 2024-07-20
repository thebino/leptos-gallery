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
use leptos::{component, view, SignalGet};
use leptos_meta::provide_meta_context;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::ActionForm;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;

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

    dbg!(auth.clone().current_user);

    Ok(auth.current_user)
}

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();

    let user = create_resource(
        move || {
            (
                login.version().get(),
                logout.version().get(),
            )
        },
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
                                view! {
                                    <p>{format!("Login error: {}", e)}</p>
                                }
                                .into_view()
                            }

                            Ok(None) => {
                                view! {
                                    <p>Please enter your <b>ALBUMCODE</b> and <b>PASSCODE</b>.</p>
                                    <ActionForm action=login>
                                        <label>
                                            "Albumcode: "
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
                                            "Password: "
                                            <input type="passcode" placeholder="Passcode" name="passcode" class="auth-input"/>
                                        </label>
                                        <br/>
                                        <input type="hidden" name="id"/>
                                        <input type="submit" value="Open album"/>
                                    </ActionForm>
                                }
                                .into_view()
                            }

                            Ok(Some(user)) => {
                                // album.update = user.username;i
                                let (album, _set_album) = create_signal(user.username);

                                view! {
                                    <div class="selected_album">{format!("Selected album: {}", album.get())} </div><Logout action=logout/>
                                    <PhotoGrid album={album} />
                                    // <h2>"Cart"</h2>
                                    // <CartItems cart=cart set_cart=set_cart />
                                    // <button on:click=checkout>"Checkout"</button>
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

#[server(Login, "/api")]
pub async fn login(
    albumcode: String,
    passcode: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let pool = pool()?;
    let auth = auth()?;

    let user = Album::validate_credentials(albumcode, passcode, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("Albumcode not found!"))?;

    dbg!(user.clone());

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());
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
        <ActionForm action=action>
            <h1>"Log In"</h1>
            <label>
                "Albumcode:"
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
                "Passcode:"
                <input type="passcode" placeholder="Passcode" name="passcode" class="auth-input"/>
            </label>
            <br/>
            <button type="submit" class="button">
                "Log In"
            </button>
        </ActionForm>
    }
}

#[component]
pub fn Logout(action: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <button type="submit" class="button">
                "Change album"
            </button>
        </ActionForm>
    }
}
