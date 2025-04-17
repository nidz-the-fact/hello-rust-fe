use dioxus::prelude::*;
use components::{ hero::Hero, navbar::Navbar};

const FAVICON: Asset = asset!("/assets/rust-2.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Nidz Rustacean!" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Navbar {}
        Hero {}
    }
}
