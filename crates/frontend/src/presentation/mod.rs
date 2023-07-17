#![allow( non_snake_case )]
#![allow( clippy::module_name_repetitions )]

use leptos::{component, create_signal, IntoView, Scope, SignalUpdate, view, Errors};
use leptos_meta::{Title, provide_meta_context, Style, Stylesheet, Meta, Html};
use leptos_router::Router;

pub mod layout;
pub mod routes;
pub mod components;

use components::error_template::AppErrorComponent;
use crate::presentation::components::error_template::ErrorComponent;
use routes::ComponentRouter;
use crate::presentation::layout::{ComponentFooter, ComponentHeader};

#[component]
pub fn AppComponent(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Html lang="en" />

        <Title text="Welcome to Leptos"/>
        <Meta name="description" content="Leptos is a web framework for Rust." />

        <TailwindStyle />

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppErrorComponent::NotFound);
            view! { cx,
                <ErrorComponent outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <ComponentHeader />
                <ComponentRouter />
                <ComponentFooter />
            </main>
        </Router>
    }
}


// #[component]
// fn TailwindStyle( _cx: Scope) -> impl IntoView {
//     view! { cx, }
// }

// #[component]
// fn TailwindStyle(cx: Scope) -> impl IntoView {
//     view! { cx,
//         <Stylesheet href="/static/frontend.css" />
//     }
// }

#[component]
fn TailwindStyle(cx: Scope) -> impl IntoView {
    let style = include_str!("../../styles/dist/tailwind.css");
    view! { cx,
        <Style>
            {style}
        </Style>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
