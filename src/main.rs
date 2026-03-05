use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

mod custom_hooks;
mod problem;
mod solutions;

use custom_hooks::use_block_blast;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone)]
pub struct MainBoardStruct {
    board: Signal<Vec<Vec<usize>>>,
    pieces: Signal<Vec<Vec<Vec<usize>>>>,
    solutions: Signal<Option<Vec<Result>>>,
}

pub struct Result {
    updated_grid: Vec<Vec<usize>>,
    painted_grid: Vec<Vec<usize>>,
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut board = use_signal(|| vec![vec![0; 8]; 8]);
    let mut pieces = use_signal(|| vec![vec![vec![0; 5]; 5]; 3]);
    let mut solutions = use_signal(|| None::<Vec<Result>>);

    use_context_provider(|| MainBoardStruct {
        board: board,
        pieces: pieces,
        solutions: solutions,
    });

    let on_solve = use_block_blast();

    let on_reset = move |_| {
        board.set(vec![vec![0; 8]; 8]);
        pieces.set(vec![vec![vec![0; 5]; 5]; 3]);
        solutions.set(None);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen bg-neutral-950 text-neutral-100 flex flex-row items-start justify-center p-4 sm:p-8 gap-8 xl:gap-12 font-sans overflow-x-hidden flex-wrap",
            div { class: "flex flex-col items-center justify-center gap-6 sm:gap-10 w-full xl:w-auto",
                problem::Problem {}
                div { class: "flex flex-row items-center justify-center gap-4 sm:gap-8 flex-wrap",
                    button {
                        class: "cursor-pointer px-6 py-3 sm:px-8 sm:py-4 bg-gradient-to-br from-emerald-400 to-emerald-600 text-white text-base sm:text-lg font-bold tracking-wider rounded-xl shadow-[0_0_15px_rgba(16,185,129,0.3)] hover:shadow-[0_0_25px_rgba(16,185,129,0.5)] hover:scale-105 transition-all duration-300 border-t border-emerald-300 min-w-[140px]",
                        onclick: on_solve,
                        "SOLVE"
                    }
                    button {
                        class: "cursor-pointer px-6 py-3 sm:px-8 sm:py-4 bg-gradient-to-br from-rose-500 to-rose-700 text-white text-base sm:text-lg font-bold tracking-wider rounded-xl shadow-[0_0_15px_rgba(225,29,72,0.3)] hover:shadow-[0_0_25px_rgba(225,29,72,0.5)] hover:scale-105 transition-all duration-300 border-t border-rose-400 min-w-[140px]",
                        onclick: on_reset,
                        "RESET"
                    }
                }
            }
            solutions::Solution {}
        }
    }
}
