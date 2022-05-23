// custom exercise example
// use of closure as traits in struct Cacher

use std::thread;
use std::time::Duration;
use std::collections::HashMap;


struct Cacher<T>
	where T: Fn(i32) -> i32
{
		calculation: T,
        hashmap: HashMap<i32, i32>,
}
	

impl<T> Cacher<T> 
	where T: Fn(i32) -> i32 
{
	fn new(calculation: T) -> Cacher<T> {
		Cacher {
			calculation,
            hashmap: HashMap::new(),
		}
	}
	
	fn value(&mut self, arg: i32) -> i32 {
        let value = self.hashmap.get(&arg);
		match value {
			Some(v) => *v,
			None => {
				let v = (self.calculation)(arg);
                self.hashmap.insert(arg, v);
				v
			},
		}
	}
}


fn generate_workout(intensity: i32, random_number: i32)
{
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", 
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}


fn closure_capture_environment() {
    let x = 4;
    let equal_to_x = move |z| z==x;
    let y = 4;
    println!("x is {:?}", x);
    assert!(equal_to_x(y));
}


fn main() {
    generate_workout(100, 2);
    closure_capture_environment();
}



#[cfg(test)]
mod tests {
	#[test]
	fn call_with_different_values() {
		use crate::Cacher;
		let mut c = Cacher::new(|a| a);
		let _v1 = c.value(1);
		let v2 = c.value(2);
		assert_eq!(v2, 2);
	}
}