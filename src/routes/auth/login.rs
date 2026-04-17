use crate::functions;
use leptos::prelude::*;
use leptos::{IntoView, component, server::ServerAction, view};
use leptos_meta::*;

#[component]
pub fn Login(action: ServerAction<functions::auth::Login>) -> impl IntoView {
    view! {
        <Meta property="og:title" content="Login" />
        <Title text="Login" />
        <Meta name="description" content="Login to the site" />
        <Meta property="og:description" content="Login to the site" />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg" />
        <div class="auth">
            <div class="auth__header">
                <h1 class="auth__heading">"Login"</h1>
            </div>
            <div class="auth__body content">
                <ActionForm attr:id="auth__login-form" attr:class="auth__form" action=action>
                    <div class="auth__form-field-set">
                        <label for="username" class="auth__form-label">
                            "Username"
                        </label>
                        <input
                            id="username"
                            required
                            name="username"
                            type="username"
                            aria-describedby="username-error"
                        />
                    </div>
                    <div class="auth__form-field-set">
                        <label for="password" class="auth__form-label">
                            "Password"
                        </label>
                        <input
                            id="password"
                            name="password"
                            type="password"
                            autocomplete="current-password"
                            aria-describedby="password-error"
                        />
                    </div>
                    <button type="submit" id="auth__login-form-submit">
                        "Log in"
                    </button>

                    <div id="auth__form-aside">
                        <div class="auth__login-form_aside_row">
                            <input id="remember" name="remember" type="checkbox" />
                            <label for="remember">"Remember me"</label>
                        </div>
                        <div class="auth__form_aside_row">
                            "Don't have an account?" <a href="/signup">"Sign up"</a>
                        </div>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}
