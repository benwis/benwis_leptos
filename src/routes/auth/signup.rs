use leptos::*;
use leptos_router::*;
use crate::functions::auth::Signup;

#[component]
pub fn Join(cx: Scope, action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        cx,
        <div class="flex min-h-full flex-col justify-center">
            <div class="mx-auto w-full max-w-md px-8">
                <h1 class="mb-4 text-3xl text-center font-bold tracking-tight text-black dark:text-white md:text-5xl">"Join"</h1>
                <ActionForm action=action class="space-y-6">
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
                        // autoFocus={true}
                        name="username"
                        type="text"
                        autoComplete="username"
                        // aria-invalid={actionData?.errors?.email ? true : undefined}
                        aria-describedby="username-error"
                        class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                    />
                    // {actionData?.errors?.email && (
                    //     <div class="pt-1 text-red-700" id="email-error">
                    //     {actionData.errors.email}
                    //     </div>
                    // )}
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
                        autoComplete="new-password"
                        // aria-invalid={actionData?.errors?.password ? true : undefined}
                        aria-describedby="password-error"
                        class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                    />
                    // {actionData?.errors?.password && (
                    //     <div class="pt-1 text-red-700" id="password-error">
                    //     {actionData.errors.password}
                    //     </div>
                    // )}
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
                    autoComplete="password_confirmation"
                    // aria-invalid={actionData?.errors?.password ? true : undefined}
                    aria-describedby="password_confirmation_error"
                    class="w-full rounded border border-gray-500 px-2 py-1 text-lg"
                />
                // {actionData?.errors?.password && (
                //     <div class="pt-1 text-red-700" id="password-error">
                //     {actionData.errors.password}
                //     </div>
                // )}
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
                    <a
                        class="text-blue-500 underline"
                        href="/signup"
                    >
                        "Log in"
                    </a>
                    </div>
                </div>
                </ActionForm>
            </div>
            </div>
    }
}