struct Rectangle{
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area is {}", area(&rect1));
    println!("rectangle is {}, area is {}", rect1.width, rect1.area());
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}
