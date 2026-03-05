use dioxus::prelude::*;
fn test() {
    let mut board = use_signal(|| vec![vec![false; 8]; 8]);
    let mut board_copy = board; // see if it's Copy
}
