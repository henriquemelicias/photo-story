#![allow( non_snake_case )]
#![allow( clippy::module_name_repetitions )]

use leptos::{component, create_signal, view, Errors, IntoView, Scope, SignalUpdate};
use leptos_router::Router;

pub mod components;
pub mod layout;
pub mod routes;

use routes::ComponentRouter;

use crate::presentation::layout::{ComponentFooter, ComponentHeader};

#[component]
pub fn AppComponent( cx: Scope ) -> impl IntoView {
    view! {
        cx,
        // content for this welcome page
        <Router>
            <main>
                <ComponentHeader />
                <AddPhotoForm />
            </main>
        </Router>
    }
}

#[component]
pub fn AddPhotoForm( cx: Scope ) -> impl IntoView {
    use leptos::{create_node_ref, event_target_value, html::Input, NodeRef, Show, SignalWith, CollectView};
    use web_sys::SubmitEvent;

    let ( get_url, set_url ) = create_signal( cx, String::new() );

    let ( get_title, set_title ) = create_signal( cx, String::new() );
    let ( get_title_errors, set_title_errors ) = create_signal( cx, Vec::<&str>::new() );

    let ( get_description, set_description ) = create_signal( cx, String::new() );

    let url_input: NodeRef<Input> = create_node_ref( cx );
    let description_input: NodeRef<Input> = create_node_ref( cx );

    let on_submit = move |ev: SubmitEvent| {
        // Don't reload the page.
        ev.prevent_default();

        // Extract the values from the form.
        let url: String = url_input().unwrap().value();
        let description: String = description_input().unwrap().value();

        set_url( url );
        set_description( description );
    };

    view! {
        cx,
        <form on:submit=on_submit>
            <label class="label">
                <span class="label-text">"Url"</span>
            </label>
            <input type="text" placeholder="Url" class="w-full max-w-xs input input-bordered"
                value=get_url
                node_ref=url_input
            />
            <label class="label">
                <span class="label-text">"Title"</span>
            </label>
            <input type="text" placeholder="Title" class="w-full max-w-xs input input-bordered"
                class=("input-success", move || !get_title.with( String::is_empty ) )
                class=("input-error", move || !get_title_errors.with( Vec::is_empty ) )
                on:change=move |ev| {
                    let value: String = event_target_value( &ev );
                    set_title_errors( Vec::new() );

                    if value.is_empty() {
                        set_title_errors.update( |errors| errors.push( "Title is required" ) );
                    }

                    if get_title_errors.with( Vec::is_empty ) {
                        set_title( value );
                    }
                }
                prop:value=get_title
            />
            <Show
                when=move || { !get_title_errors.with( Vec::is_empty ) }
                fallback=|_| {}
            >
                <div class="alert alert-error">
                    <ul>
                    {
                        get_title_errors()
                            .into_iter()
                            .map( |n| view! { cx, <li>{n}</li> } )
                            .collect_view( cx )
                    }
                    </ul>
                </div>
            </Show>
            <label class="label">
                <span class="label-text">"Description"</span>
            </label>
            <input type="text" placeholder="Description" class="w-full max-w-xs input input-bordered"
                value=get_description
                node_ref=description_input
            />
            <button type="submit" class="btn">"Submit"</button>
        </form>
    }
}
