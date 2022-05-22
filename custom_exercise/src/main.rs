use std::thread;
use std::time::Duration;


struct Cacher<T>
	where T: Fn(i32) -> i32 
{
		calculation: T,
		value: Option<i32>,
}
	

impl<T> Cacher<T> 
	where T: Fn(i32) -> i32 
{
	fn new(calculation: T) -> Cacher<T> {
		Cacher {
			calculation,
			value: None,
		}
	}
	
	fn value(&mut self, arg: i32) -> i32 {
		match self.value {
			Some(v) => v,
			None => {
				let v = (self.calculation)(arg);
				self.value = Some(v);
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


fn main() {
    generate_workout(100, 2);
}



#[cfg(test)]
mod tests {
	#[test]
	fn call_with_different_values() {
		use crate::Cacher;
		let mut c = Cacher::new(|a| a);
		let v1 = c.value(1);
		let v2 = c.value(2);
		assert_eq!(v2, 2);
	}
}