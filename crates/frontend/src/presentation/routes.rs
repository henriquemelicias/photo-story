// use crate::presentation::models;

use leptos::{component, view, IntoView };
use leptos_router::{Route, Routes};

#[must_use]
#[component]
pub fn ComponentRouter() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view=|| view! { <Home/> }/>
            // <Route path="" view=|cx| view! { cx, <NotFound/> }/>
        </Routes>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h1 class="text-9xl font-bold underline">

            "Home"
            <br/>
            <a href="/hello-server">
                "click here to go to hello-server"
            </a>
        </h1>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! { <h1>"404"</h1> }
}
