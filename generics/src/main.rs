// example code for generics, traits and lifetimes

fn main() {
    //largest_simple();
	largest_duplicate();
}


fn largest_simple() {
	
	let number_list = vec![34, 50, 25, 100, 65];
	
	let mut largest = number_list[0];
	
	for number in number_list {
		if number > largest {
			largest = number;
		};
		println!("{}", number);
	}
	
	println!("largest number is {}", largest);
}


fn largest_duplicate() {
	
	let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("largest number is {}", result);
	
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let result = largest(&number_list);
	println!("largest number is {}", result);
	
}


fn largest(list: &[i32]) -> i32 {
	
	let mut largest = list[0];
	
	for &item in list {
		if item > largest {
			largest = item;
		};
		println!("{}", item);
	}
	
	largest
}