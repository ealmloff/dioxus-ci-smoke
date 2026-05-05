use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        main {
            h1 { "Dioxus CI smoke test" }
            p { "Preview workflow exercised successfully without apt or Firefox setup in the reusable workflow." }
        }
    }
}
