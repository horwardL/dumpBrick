use dioxus::prelude::*;

mod board;
mod pieces;

#[component]
pub fn Problem() -> Element {
    rsx! {
        div {
            id: "problem",
            class: "flex flex-col items-center justify-center p-3 sm:p-6 md:p-8 bg-neutral-900/80 rounded-2xl sm:rounded-3xl shadow-2xl border border-neutral-800 backdrop-blur-md self-center xl:self-start mt-4 xl:mt-8 w-full max-w-fit mx-auto",
            board::MainBoard {}
            pieces::Pieces {}
        }
    }
}
