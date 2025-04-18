use dioxus::prelude::*;

const RUST_LOGO: Asset = asset!("/assets/rust.png");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-zinc-800 shadow-md text-white",
            div { class: "max-w-screen-xl mx-auto p-4 flex justify-between items-center",
                a {
                    href: "#",
                    class: "flex items-center space-x-2",
                    img {
                        class: "h-8 w-8",
                        src: RUST_LOGO,
                    }
                    span {
                        class: "text-xl font-bold",
                        "Rustacean"
                    }
                }
                ul { class: "flex space-x-6 text-sm",
                    li {
                        a {
                            href: "https://github.com/nidz-the-fact",
                            class: "hover:text-orange-400", "Contact"
                        }
                    }
                }
            }
        }
    }
}
