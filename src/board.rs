use dioxus::prelude::*;

#[component]
pub fn MainBoard() -> Element {
    rsx! {
        div {
            id: "main-board",
            class: "flex flex-col items-center justify-center p-8",
            div { class: "grid grid-cols-8 gap-2 border-2 border-gray-800 p-2 bg-gray-100 rounded",
                for i in 0..8 {
                    for j in 0..8 {
                        div {
                            key: "{i}-{j}",
                            class: "w-16 h-16 bg-white border border-gray-300 flex items-center justify-center text-xl font-bold hover:bg-gray-50 cursor-pointer",
                            "{i}-{j}"
                        }
                    }
                }
            }
        }
    }
}
