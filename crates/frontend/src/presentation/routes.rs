// use crate::presentation::models;

use leptos::{component, view, IntoView, Scope};
use leptos_router::{Route, Routes};

#[must_use]
#[component]
pub fn ComponentRouter( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <Routes>
            <Route path="/" view=|cx| view! { cx, <Home/> }/>
            // <Route path="" view=|cx| view! { cx, <NotFound/> }/>
        </Routes>
    }
}

#[component]
fn Home( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

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
fn NotFound( cx: Scope ) -> impl IntoView
{
    view! { cx, <h1>"404"</h1> }
}
