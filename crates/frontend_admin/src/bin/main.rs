use leptos::{mount_to_body, view};

use frontend_admin::presentation::AppComponent;

pub fn main() { mount_to_body( |cx| view! { cx, <AppComponent /> } ); }
