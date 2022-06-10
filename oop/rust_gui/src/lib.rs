pub trait Draw {  // a trait object
    fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button {:?}", self);
        // code to actually draw a button
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // use a trait object
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}