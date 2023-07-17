use leptos::{component, view, IntoView, Scope};

#[must_use]
#[component]
pub fn ComponentHeader( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <header>
            <Navbar />
        </header>
    }
}

#[component]
fn Navbar( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <nav class="navbar bg-base-100">

            <NavbarStart />
            <NavbarCenter />
            <NavbarEnd />
        </nav>
    }
}

#[component]
fn NavbarStart( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <div class="navbar-start">
            <NavbarDropdownMenu />
            <a class="btn btn-ghost text-xl normal-case">
                "DaisyUI"
            </a>
        </div>
    }
}

#[component]
fn NavbarDropdownMenu( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <div class="dropdown">

            <label class="btn btn-ghost lg:hidden" tabindex="0">
                <SvgNavbarDropdownMenu />
            </label>

            // Menu.
            <ul
                class="menu menu-compact dropdown-content bg-base-100 rounded-box mt-3 w-52 p-2 shadow"
                tabindex="0"
            >

                // Menu entry 1.
                <li><a>"Item 1"</a></li>
                // Menu entry 2: submenu.
                <li tabindex="0">

                    <a class="justify-between">
                        "Parent"
                        <SvgNavbarMenuEntry />
                    </a>

                    <ul class="p-2">
                        <li><a>"Submenu Item 1"</a></li>
                        <li><a>"Submenu Item 2"</a></li>
                    </ul>
                </li>
                // Menu entry 3.
                <li><a>"Item 3"</a></li>
            </ul>
        </div>
    }
}

#[component]
fn SvgNavbarDropdownMenu( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <svg class="h-5 w-5" fill="none" stroke="currentColor" view_box="0 0 24 24">
            <path stroke_linecap="round" stroke_linejoin="round" stroke_width="2" d="M4 6h16M4 12h8m-8 6h16"/>
        </svg>
    }
}

#[component]
fn SvgNavbarMenuEntry( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <svg class="fill-current" width="24" height="24" view_box="0 0 24 24">
            <path d="M8.59,16.58L13.17,12L8.59,7.41L10,6L16,12L10,18L8.59,16.58Z"/>
        </svg>
    }
}

#[component]
fn NavbarCenter( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <div class="navbar-center hidden lg:flex">
            <NavbarHorizontalMenu />
        </div>
    }
}

#[component]
fn NavbarHorizontalMenu( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <ul class="menu menu-horizontal px-1">

            // Menu entry 1.
            <li><a>"Item 1"</a></li>
            // Menu entry 2: submenu.
            <li tabindex="0">
                <a>
                    "Parent"
                    <SvgNavbarMenuEntry />
                </a>

                <ul class="p-2">
                    <li><a>"Submenu Item 1"</a></li>
                    <li><a>"Submenu Item 2"</a></li>
                </ul>
            </li>
            // Menu entry 3.
            <li><a>"Item 3"</a></li>
        </ul>
    }
}

#[component]
fn NavbarEnd( cx: Scope ) -> impl IntoView
{
    view! {
        cx,

        <div class="navbar-end">
            <a class="btn">
                "Get started"
            </a>
        </div>
    }
}
