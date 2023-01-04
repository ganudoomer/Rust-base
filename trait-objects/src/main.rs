use std::vec;

use trait_objects::{Button, Draw, Screen};

struct SelectBox {
    w: u32,
    h: u32,
    opts: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

fn main() {
    println!("Hello, world!");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                w: 100,
                h: 200,
                opts: vec![String::from("Hey"), String::from("Hey")],
            }),
            Box::new(Button {
                height: 300,
                width: 300,
                label: String::from("Hey"),
            }),
        ],
    };
}
