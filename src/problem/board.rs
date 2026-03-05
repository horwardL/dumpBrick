use dioxus::prelude::*;

use crate::MainBoardStruct;

#[component]
pub fn MainBoard() -> Element {
    let mut board = use_context::<MainBoardStruct>().board;
    let mut handled_mousedown = use_signal(|| false);

    let mut on_click = move |i: usize, j: usize| {
        let mut b = board.write();
        b[i][j] = (b[i][j] + 1) % 2;
    };

    rsx! {
        div {
            id: "main-board",
            class: "flex flex-col items-center justify-center mb-6 sm:mb-8",
            div { class: "grid grid-cols-8 gap-1 sm:gap-1.5 p-2 sm:p-4 bg-neutral-950/50 rounded-xl sm:rounded-2xl shadow-inner border border-neutral-800/80",
                for i in 0..board.read().len() {
                    for j in 0..board.read()[i].len() {
                        {
                            let is_active: bool = board.read()[i][j] == 1;
                            let bg_class = if is_active {
                                "bg-gradient-to-br from-blue-400 to-blue-600 shadow-[0_0_10px_rgba(59,130,246,0.4)] border-blue-400 z-10"
                            } else {
                                "bg-neutral-800/50 hover:bg-neutral-700/60 border-neutral-700/80"
                            };
                            rsx! {
                                div {
                                    key: "{i}-{j}",
                                    "data-grid-coord": "{i}-{j}",
                                    class: "w-6 h-6 min-[380px]:w-8 min-[380px]:h-8 sm:w-10 sm:h-10 border rounded flex items-center justify-center transition-all duration-200 cursor-pointer select-none {bg_class}",
                                    pointer_events: "auto",
                                    style: "touch-action: none;",
                                    onclick: move |_| {
                                        if handled_mousedown() {
                                            handled_mousedown.set(false);
                                        } else {
                                            on_click(i, j);
                                        }
                                    },
                                    onmousedown: move |_| {
                                        handled_mousedown.set(true);
                                        on_click(i, j);
                                    },
                                    onmouseout: move |_| {
                                        handled_mousedown.set(false);
                                    },
                                    onmouseenter: move |e| {
                                        if e.held_buttons().contains(dioxus::html::input_data::MouseButton::Primary) {
                                            let mut b = board.write();
                                            b[i][j] = 1;
                                        }
                                    },
                                    ontouchstart: move |_| {
                                        handled_mousedown.set(true);
                                        on_click(i, j);
                                    },
                                    ontouchmove: move |e| {
                                        if let Some(touch) = e.touches().first() {
                                            let x = touch.page_coordinates().x;
                                            let y = touch.page_coordinates().y;


                                            if let Some(window) = web_sys::window() {
                                                if let Some(document) = window.document() {
                                                    if let Some(elem) = document.element_from_point(x as f32, y as f32) {
                                                        if let Some(id_str) = elem.get_attribute("data-grid-coord") {
                                                            let parts: Vec<&str> = id_str.split('-').collect();
                                                            if parts.len() == 2 {
                                                                if let (Ok(row), Ok(col)) = (
                                                                    parts[0].parse::<usize>(),
                                                                    parts[1].parse::<usize>(),
                                                                ) {
                                                                    let mut b = board.write();
                                                                    if row < b.len() && col < b[row].len() {
                                                                        b[row][col] = 1;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
