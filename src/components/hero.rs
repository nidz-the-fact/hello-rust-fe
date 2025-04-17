use dioxus::prelude::*;

const CRAB_1: Asset = asset!("/assets/rust-1.svg");
const CRAB_2: Asset = asset!("/assets/rust-2.svg");
const CRAB_3: Asset = asset!("/assets/rust-3.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "motion-preset-fade  bg-zinc-900 text-orange-500 flex flex-col items-center justify-center min-h-screen text-center px-4",
            h1 { 
                class: "text-[120px] font-bold mb-2 drop-shadow-lg ", 
                "Hello Rust" 
            }
            h2 { 
                class: "text-[30px] text-yellow-400 mb-6", 
                "Nidz Rustacean!" 
            }
            div { class: "flex gap-8 mt-6 animate-fade-in",
                img {
                    alt: "Rust Crab 1",
                    class: "w-36 hover:scale-110 hover:rotate-3 transition-transform duration-300",
                    src: CRAB_1,
                }
                img {
                    alt: "Rust Crab 2",
                    class: "w-28 hover:scale-110 hover:rotate-3 transition-transform duration-300",
                    src: CRAB_2,
                }
                img {
                    alt: "Rust Crab 3",
                    class: "w-36 hover:scale-110 hover:-rotate-3 transition-transform duration-300",
                    src: CRAB_3,
                }
            }
        }
    }
}
