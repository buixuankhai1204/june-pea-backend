use leptos::prelude::*;

use crate::state::auth::AuthState;

/// Wraps protected routes. Redirects to /login if not authenticated.
#[component]
pub fn AuthGuard(children: ChildrenFn) -> impl IntoView {
    let auth = expect_context::<AuthState>();

    view! {
        {move || {
            if auth.user.get().is_some() {
                children().into_any()
            } else {
                // Redirect to login
                let navigate = leptos_router::hooks::use_navigate();
                navigate("/login", Default::default());
                view! { <div>"Redirecting..."</div> }.into_any()
            }
        }}
    }
}
