extern crate averaged_collection;
extern crate rust_gui;
use averaged_collection::AveragedCollection;
use rust_gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a select box
    }
}

use rust_gui::{Screen, Button};

fn main() {
    println!("Object-Oriented Programming in Rust: Examples");

    let mut ac = AveragedCollection::new(vec![], 0.0);
    ac.add(1);
    ac.add(2);
    println!("{:?}, average = {}", ac, ac.average());
    ac.remove();
    println!("{:?}, average = {}", ac, ac.average());
    ac.remove();
    println!("{:?}, average = {}", ac, ac.average());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
