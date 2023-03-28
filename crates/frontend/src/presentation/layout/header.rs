use dioxus::prelude::*;

pub fn ComponentHeader( cx: Scope ) -> Element
{
    cx.render( rsx!(

        header {
            Navbar {}
        }
    ) )
}

fn Navbar( cx: Scope ) -> Element
{
    cx.render( rsx!(

        nav {
            class: "navbar bg-base-100",

            NavbarStart {}
            NavbarCenter {}
            NavbarEnd {}
        }
    ) )
}

fn NavbarStart( cx: Scope ) -> Element
{
    cx.render( rsx!(

        div {
            class: "navbar-start",

            NavbarDropdownMenu {}
            a { class: "btn btn-ghost normal-case text-xl", "DaisyUI" }
        }
    ) )
}

fn NavbarDropdownMenu( cx: Scope ) -> Element
{
    cx.render( rsx!(

        div {
            class: "dropdown",

            label {
                class: "btn btn-ghost lg:hidden",
                tabindex: "0",

                SvgNavbarDropdownMenu {}
            }

            // Menu.
            ul {
                class: "menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52",
                tabindex: "0",

                // Menu entries.
                li { a { "Item 1" } }
                // Submenu.
                li {
                    tabindex: "0",

                    a {
                        class: "justify-between",

                        "Parent"
                        SvgNavbarMenuEntry {}
                    }

                    ul {
                        class: "p-2",

                        li { a { "Submenu Item 1" } }
                        li { a { "Submenu Item 2" } }
                    }
                }
                li { a { "Item 3" } }
            }
        }
    ) )
}

fn SvgNavbarDropdownMenu( cx: Scope ) -> Element
{
    cx.render( rsx!(

        svg {
            class: "h-5 w-5",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",

            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M4 6h16M4 12h8m-8 6h16"
            }
        }
    ) )
}

fn SvgNavbarMenuEntry( cx: Scope ) -> Element
{
    cx.render( rsx!(

        svg {
            class: "fill-current",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",

            path { d: "M8.59,16.58L13.17,12L8.59,7.41L10,6L16,12L10,18L8.59,16.58Z" }
        }
    ) )
}

fn NavbarCenter( cx: Scope ) -> Element
{
    cx.render( rsx!(

        div {
            class: "navbar-center hidden lg:flex",

            NavbarHorizontalMenu {}
        }
    ) )
}

fn NavbarHorizontalMenu( cx: Scope ) -> Element
{
    cx.render( rsx!(

        ul {
            class: "menu menu-horizontal px-1",

            li { a { "Item 1" } }
            li {
                tabindex: "0",

                a {
                    "Parent"
                    SvgNavbarMenuEntry {}
                }

                ul {
                    class: "p-2",

                    li { a { "Submenu Item 1" } }
                    li { a { "Submenu Item 2" } }
                }
            }
            li { a { "Item 3" } }
        }
    ) )
}

fn NavbarEnd( cx: Scope ) -> Element
{
    cx.render( rsx!(

        div {
            class: "navbar-end",

            a { class: "btn", "Get started" }
        }
    ) )
}
