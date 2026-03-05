use dioxus::prelude::*;

use crate::MainBoardStruct;

#[component]
pub fn Pieces() -> Element {
    let mut pieces = use_context::<MainBoardStruct>().pieces;
    let mut handled_mousedown = use_signal(|| false);

    let mut on_click = move |piece: usize, i: usize, j: usize| {
        let mut p = pieces.write();
        p[piece][i][j] = (p[piece][i][j] + 1) % 2;
    };

    rsx! {
        div {
            id: "pieces",
            class: "flex flex-row items-center justify-center gap-4 sm:gap-6 flex-wrap mt-2 sm:mt-4",
            for p in 0..pieces.read().len() {
                div { class: "grid grid-cols-5 gap-0.5 sm:gap-1 p-2 sm:p-3 bg-neutral-950/50 rounded-xl sm:rounded-2xl shadow-inner border border-neutral-800/80 transition-transform duration-300 hover:scale-105",
                    for i in 0..5 {
                        for j in 0..5 {
                            {
                                let is_active = pieces.read()[p][i][j] == 1;
                                let bg_class = if is_active {
                                    "bg-gradient-to-br from-blue-400 to-blue-600 shadow-[0_0_10px_rgba(59,130,246,0.4)] border-blue-400 z-10"
                                } else {
                                    "bg-neutral-800/50 hover:bg-neutral-700/60 border-neutral-700/80"
                                };
                                rsx! {
                                    div {
                                        key: "piece-{p}-{i}-{j}",
                                        "data-piece-coord": "{p}-{i}-{j}",
                                        class: "w-5 h-5 min-[380px]:w-6 min-[380px]:h-6 sm:w-8 sm:h-8 border rounded flex items-center justify-center transition-all duration-200 cursor-pointer select-none {bg_class}",
                                        pointer_events: "auto",
                                        style: "touch-action: none;",
                                        onclick: move |_| {
                                            if handled_mousedown() {
                                                handled_mousedown.set(false);
                                            } else {
                                                on_click(p, i, j);
                                            }
                                        },
                                        onmousedown: move |_| {
                                            handled_mousedown.set(true);
                                            on_click(p, i, j);
                                        },
                                        onmouseout: move |_| {
                                            handled_mousedown.set(false);
                                        },
                                        onmouseenter: move |e| {
                                            if e.held_buttons().contains(dioxus::html::input_data::MouseButton::Primary) {
                                                let mut pieces_write = pieces.write();
                                                pieces_write[p][i][j] = 1;
                                            }
                                        },
                                        ontouchstart: move |_| {
                                            handled_mousedown.set(true);
                                            on_click(p, i, j);
                                        },
                                        ontouchmove: move |e| {
                                            if let Some(touch) = e.touches().first() {
                                                let x = touch.page_coordinates().x;
                                                let y = touch.page_coordinates().y;

                                                if let Some(window) = web_sys::window() {
                                                    if let Some(document) = window.document() {
                                                        if let Some(elem) = document.element_from_point(x as f32, y as f32) {
                                                            if let Some(id_str) = elem.get_attribute("data-piece-coord") {
                                                                let parts: Vec<&str> = id_str.split('-').collect();
                                                                if parts.len() == 3 {
                                                                    if let (Ok(piece_idx), Ok(row), Ok(col)) = (
                                                                        parts[0].parse::<usize>(),
                                                                        parts[1].parse::<usize>(),
                                                                        parts[2].parse::<usize>()
                                                                    ) {
                                                                        let mut pieces_write = pieces.write();
                                                                        if piece_idx < pieces_write.len() && row < pieces_write[piece_idx].len() && col < pieces_write[piece_idx][row].len() {
                                                                            pieces_write[piece_idx][row][col] = 1;
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
}
