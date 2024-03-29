#[derive(PartialEq, Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}


impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    pub fn area(&self) -> u32 {
        self.width * self.length
    }
}


pub fn add_two(a: i32) -> i32 {
    a+2
}


pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}


struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {

        println!("OK");

        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}


#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
    }

    #[test]
    fn exploration() {
    }

    #[test]
    fn another() {
       // panic!("Make the test fail!");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_does_not_adds_three() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn rectangle_area() {
        let larger = Rectangle {length: 8, width: 7};
        assert_ne!(larger.area(), 10);
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was {}", result);
    }

    #[test]
    #[should_panic(expected="Guess value must be between")]
    fn greater_than_100() {
        Guess::new(200);
    }
}


/// Adds one to the number given
///
/// # Examples
///
/// ```
/// use adder::add_one;
/// let five = 5;
///
/// assert_eq!(6, add_one(five));
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}