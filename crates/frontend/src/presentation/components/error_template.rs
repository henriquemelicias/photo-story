use http::status::StatusCode;
use leptos::*;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppErrorComponent {
    #[error( "The resource does not exist" )]
    NotFound,
}

impl AppErrorComponent {
    pub fn status_code( &self ) -> StatusCode {
        match self {
            AppErrorComponent::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorComponent(
    cx: Scope,
    #[prop( optional )] outside_errors: Option<Errors>,
    #[prop( optional )] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some( err ) => create_rw_signal( cx, err ),
        None => match errors {
            Some( err ) => err,
            None => panic!( "No Errors found and we expected errors!" ),
        },
    };
    // Get Errors from Signal
    let errors = errors.get();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppErrorComponent> = errors
        .into_iter()
        .filter_map( |( _k, v )| v.downcast_ref::<AppErrorComponent>().cloned() )
        .collect();

    view! {cx,
        <h1>{if errors.len() > 1 {"Errors:"} else {"Error:"}}</h1>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each= move || {errors.clone().into_iter().enumerate()}
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            view= move |cx, error| {
                let error_string = error.1.to_string();
                let error_code= error.1.status_code();
                view! {
                    cx,
                    <h2>{error_code.to_string()}</h2>
                    <p>"Message: " {error_string}</p>
                }
            }
        />
    }
}
