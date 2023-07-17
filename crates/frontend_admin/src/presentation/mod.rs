#![allow( non_snake_case )]
#![allow( clippy::module_name_repetitions )]

use leptos::{component, create_signal, view, Errors, IntoView, Scope, SignalUpdate};
use leptos_router::Router;

pub mod components;
pub mod layout;
pub mod routes;

use crate::presentation::layout::{ComponentFooter, ComponentHeader};
use routes::ComponentRouter;

#[component]
pub fn AppComponent( cx: Scope ) -> impl IntoView
{
    view! {
        cx,
        // content for this welcome page
        <Router>
            <main>
                <ComponentHeader />
                <ComponentFooter />
            </main>
        </Router>
    }
}
