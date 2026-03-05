use dioxus::prelude::*;

fn test_mouse(event: Event<MouseData>) {
    let bs = event.held_buttons();
    println!("{:?}", bs);
}
