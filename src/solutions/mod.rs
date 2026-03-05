use dioxus::prelude::*;

use crate::MainBoardStruct;

#[component]
pub fn Solution() -> Element {
    let main_board_struct = use_context::<MainBoardStruct>();
    let solutions = main_board_struct.solutions.read();

    rsx! {
        div {
            class: "flex flex-row gap-4 sm:gap-6 max-h-[60vh] xl:max-h-[85vh] overflow-y-auto pr-2 pb-4 self-center xl:self-start mt-4 xl:mt-0 w-full xl:w-auto max-w-2xl flex-wrap justify-center",
            if let Some(solutions) = &*solutions {
                if solutions.is_empty() {
                    div { class: "text-neutral-400 text-base sm:text-lg font-medium p-6 sm:p-8 bg-neutral-900/50 rounded-2xl border border-neutral-800 w-full text-center tracking-wide",
                        "No solutions found."
                    }
                } else {
                    for (s , _) in solutions.iter().enumerate() {
                        div { class: "flex flex-col items-center justify-center p-3 sm:p-4 bg-neutral-950/50 rounded-2xl sm:rounded-3xl shadow-inner border border-neutral-800/80 transition-transform duration-300 hover:scale-105",
                            div { class: "text-neutral-500 font-bold text-[10px] sm:text-xs tracking-widest uppercase mb-2",
                                "Step {s + 1}"
                            }
                            {
                                let painted_grid = &solutions[s].painted_grid;
                                let updated_grid = &solutions[s].updated_grid;
                                rsx! {
                                    div { class: "flex flex-col items-center justify-center gap-2",
                                        div { class: "grid grid-cols-8 gap-0.5 sm:gap-1 p-1.5 sm:p-2 bg-neutral-900/80 rounded-lg sm:rounded-xl shadow-inner border border-neutral-800",
                                            for i in 0..painted_grid.len() {
                                                for j in 0..painted_grid[i].len() {
                                                    {
                                                        let bg_class = match painted_grid[i][j] {
                                                            1 => "bg-gradient-to-br from-blue-400 to-blue-600 shadow-[0_0_10px_rgba(59,130,246,0.4)] border-blue-400 z-10",
                                                            2 => "bg-gradient-to-br from-rose-400 to-rose-600 shadow-[0_0_10px_rgba(225,29,72,0.4)] border-rose-400 z-10",
                                                            3 => "bg-gradient-to-br from-amber-400 to-amber-600 shadow-[0_0_10px_rgba(245,158,11,0.4)] border-amber-400 z-10",
                                                            _ => "bg-neutral-800/50 border-neutral-700/80 hover:bg-neutral-700/60"
                                                        };
                                                        rsx! {
                                                            div {
                                                                key: "sol-{s}-p-{i}-{j}",
                                                                class: "w-3 h-3 min-[380px]:w-4 min-[380px]:h-4 sm:w-5 sm:h-5 rounded-[2px] border {bg_class}",
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "text-neutral-600 text-lg font-light text-center w-full my-1", "↓" }
                                        div { class: "grid grid-cols-8 gap-0.5 sm:gap-1 p-1.5 sm:p-2 bg-neutral-900/80 rounded-lg sm:rounded-xl shadow-inner border border-neutral-800",
                                            for i in 0..updated_grid.len() {
                                                for j in 0..updated_grid[i].len() {
                                                    {
                                                        let val = updated_grid[i][j];
                                                        let bg_class = if val == 1 {
                                                            "bg-gradient-to-br from-blue-400 to-blue-600 shadow-[0_0_10px_rgba(59,130,246,0.4)] border-blue-400 z-10"
                                                        } else {
                                                            "bg-neutral-800/50 border-neutral-700/80 hover:bg-neutral-700/60"
                                                        };
                                                        rsx! {
                                                            div {
                                                                key: "sol-{s}-u-{i}-{j}",
                                                                class: "w-3 h-3 min-[380px]:w-4 min-[380px]:h-4 sm:w-5 sm:h-5 rounded-[2px] border {bg_class}",
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "hidden" }
            }
        }
    }
}
