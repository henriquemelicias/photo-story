// use crate::presentation::models;

use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};

#[must_use]
pub fn ComponentRouter( cx: Scope ) -> Element
{
    cx.render( rsx!(

        Router
        {
            Route { to: "/", ComponentHome {} }
            Route { to: "", ComponentNotFound {} }

        }
    ) )
}

fn ComponentHome( cx: Scope ) -> Element
{
    cx.render( rsx!(

        h1
        {
            class: "text-9xl font-bold underline",

            "Home"
            br {}
            Link { to: "/hello-server", "click here to go to hello-server" }
        }
    ) )
}

fn ComponentNotFound( cx: Scope ) -> Element
{
    cx.render( rsx!(

        h1 { "404" }
    ) )
}
