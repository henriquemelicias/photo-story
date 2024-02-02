use leptos::{component, view, IntoView };

#[must_use]
#[component]
pub fn ComponentFooter() -> impl IntoView {
    view! { <FooterTop/> }
}

#[component]
fn FooterTop() -> impl IntoView {
    view! {
        <footer class="footer bg-base-200 text-base-content flex flex-wrap justify-between p-10">

            <div>
                <span class="footer-title">"Services"</span>
                <a class="link link-hover">"Branding"</a>
                <a class="link link-hover">"Marketing"</a>
                <a class="link link-hover">"Design"</a>
                <a class="link link-hover">"Development"</a>
            </div>

            <div>
                <span class="footer-title">"Company"</span>
                <a class="link link-hover">"About us"</a>
                <a class="link link-hover">"Contact"</a>
                <a class="link link-hover">"Jobs"</a>
                <a class="link link-hover">"Press Kit"</a>
            </div>

            <div>
                <span class="footer-title">"Legal"</span>
                <a class="link link-hover">"Terms of Service"</a>
                <a class="link link-hover">"Privacy Policy"</a>
                <a class="link link-hover">"Cookie Policy"</a>
            </div>

            <div>
                <span class="footer-title">"Newsletter"</span>
                <div class="form-control w-100">
                    <label class="label">
                        <span class="label-text">"Enter your email address"</span>
                    </label>

                    <div class="relative">
                        <input class="input input-bordered w-100" placeholder="Email address"></input>

                        <button
                            class="btn btn-primary btn-sm absolute"
                            style="right: 0.5rem; top: 0.5rem;"
                        >
                            "Subscribe"
                        </button>
                    </div>
                </div>
            </div>
        </footer>
    }
}
