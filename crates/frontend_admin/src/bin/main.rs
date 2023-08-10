use frontend_admin::presentation::AppComponent;
use leptos::{mount_to_body, view};

pub fn main() { mount_to_body( |cx| view! { cx, <AppComponent /> } ); }
