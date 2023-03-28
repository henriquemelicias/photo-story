use dioxus::prelude::*;

pub fn ComponentFooter( cx: Scope ) -> Element { cx.render( rsx!( FooterTop {} ) ) }

fn FooterTop( cx: Scope ) -> Element
{
    cx.render( rsx!(

        footer {
            class: "footer p-10 bg-base-200 text-base-content flex justify-between flex-wrap",

            div {
                span { class: "footer-title", "Services" }
                a { class: "link link-hover", "Branding" }
                a { class: "link link-hover", "Marketing" }
                a { class: "link link-hover", "Design" }
                a { class: "link link-hover", "Development" }
            }

            div {
                span { class: "footer-title", "Company" }
                a { class: "link link-hover", "About us" }
                a { class: "link link-hover", "Contact" }
                a { class: "link link-hover", "Jobs" }
                a { class: "link link-hover", "Press Kit" }
            }

            div {
                span { class: "footer-title", "Legal" }
                a { class: "link link-hover", "Terms of Service" }
                a { class: "link link-hover", "Privacy Policy" }
                a { class: "link link-hover", "Cookie Policy" }
            }

            div {
                span { class: "footer-title", "Newsletter" }
                div {
                    class: "form-control w-100",

                    label {
                        class: "label",

                        span { class: "label-text", "Enter your email address" }
                    }

                    div {
                        class: "relative",

                        input {
                            class: "input input-bordered w-100",
                            placeholder: "Email address",
                        }

                        button {
                            class: "absolute btn btn-primary btn-sm",
                            style: "right: 0.5rem; top: 0.5rem;",

                            "Subscribe"
                        }
                    }
                }
            }
        }
    ) )
}
