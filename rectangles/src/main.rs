struct Rectangle{
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("area is {}", area(&rect1));
    println!("rectangle1 is {}, area is {}", rect1.width, rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
