use sycamore::prelude::*;

#[component]
pub fn Nav() -> View {
    view! {
        nav {
            a(href="/") { "Home" }
            a(href="/counter") { "Counter" }
            a(href="/blog") { "Blog" }
        }
    }
}
