use crate::functions::auth::Signup;
use leptos::prelude::*;
use leptos::server::ServerAction;
use leptos::{component, view, ActionForm, IntoView};
use leptos_meta::*;

#[component]
pub fn Join(action: ServerAction<Signup>) -> impl IntoView {
    view! {
        <Meta property="og:title" content="Signup"/>
        <Title text="Signup"/>
        <Meta name="description" content="Signup for the site"/>
        <Meta property="og:description" content="Signup for the site"/>
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
        <div class="flex min-h-full flex-col justify-center">
            <div class="mx-auto w-full max-w-md px-8">
                <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">
                    "Join"
                </h1>
                <ActionForm action=action> // TODO class="space-y-6">
                    <div>
                        <label
                            for="email"
                            class="block text-sm font-medium text-gray-700 dark:text-white"
                        >
                            "Username"
                        </label>
                        <div class="mt-1">
                            <input
                                id="username"
                                required
                                name="username"
                                type="text"
                                autocomplete="username"
                                aria-describedby="username-error"
                                class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                            />
                        </div>
                    </div>
                    <div>
                        <label
                            for="email"
                            class="block text-sm font-medium text-gray-700 dark:text-white"
                        >
                            "Diplayed Name"
                        </label>
                        <div class="mt-1">
                            <input
                                id="display_name"
                                required
                                name="display_name"
                                type="text"
                                autocomplete="dipslay_name"
                                aria-describedby="display_name-error"
                                class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                            />
                        </div>
                    </div>
                    <div>
                        <label
                            for="password"
                            class="block text-sm font-medium text-gray-700 dark:text-white"
                        >
                            "Password"
                        </label>
                        <div class="mt-1">
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autocomplete="new-password"
                                aria-describedby="password-error"
                                class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                            />
                        </div>
                    </div>
                    <div>
                        <label
                            for="password_confirmation"
                            class="block text-sm font-medium text-gray-700 dark:text-white"
                        >
                            "Confirm Password"
                        </label>
                        <div class="mt-1">
                            <input
                                id="password_confirmation"
                                name="password_confirmation"
                                type="password"
                                autocomplete="password_confirmation"
                                aria-describedby="password_confirmation_error"
                                class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                            />
                        </div>
                    </div>
                    <button
                        type="submit"
                        class="w-full rounded bg-yellow-400  py-2 px-4 text-white dark:text-gray-700 hover:bg-yellow-600 focus:bg-yellow-500"
                    >
                        "Create Account"
                    </button>
                    <div class="flex items-center justify-center">
                        <div class="text-center text-sm text-gray-500">
                            "Already have an account?"
                            <a class="text-blue-500 underline" href="/signup">
                                "Log in"
                            </a>
                        </div>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}
