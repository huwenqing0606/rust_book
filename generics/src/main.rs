// example code for generics, traits and lifetimes

fn main() {
    largest_simple();
	largest_duplicate();
	use_struct();
	use_trait();
	lifetimes1();
	lifetimes2();
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
	
	let char_list = vec!['a', 's', 'd', 'f'];
	let result = largest_generic(&char_list);
	println!("largest char is {}", result)
	
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


fn largest_generic<T: PartialOrd + Copy + std::fmt::Display>(list: &[T]) -> T {
	
	let mut largest = list[0];
	
	for &item in list {
		if item > largest {
			largest = item;
		};
		println!("{}", item);
	}
	
	largest
}


struct Point<T,U> {
	x: T,
	y: U,
}

impl<A, U> Point<A, U> {
	fn x_T(&self) -> &A {
		&self.x
	}
}

impl Point<f32, f32> {
	fn x_f32(&self) -> &f32 {
		&self.x
	}
}

impl<x1, y1> Point<x1, y1> {
	fn mixup<x2, y2>(self, other: Point<x2, y2>) -> Point<x1, y2> {
		Point{x: self.x, y: other.y}
	}
}

fn use_struct() {
	let both_integer = Point{x: 5, y: 10};
	let both_float = Point{x: 1.0, y: 4.0};
	let integer_and_float = Point{x: 5, y: 4.0};
	println!("integer_and_float.x = {}", integer_and_float.x_T());
	println!("both_float.x = {}", both_float.x_f32());
	let mix = both_float.mixup(both_integer);
	println!("mixup = {} {}", mix.x, mix.y);
}


use generics::aggregator::{Summary, Tweet, NewsArticle}; 

fn use_trait() {
	let tweet = Tweet {
		username: String::from("Dr_HuWenqing"),
		content: String::from("Hello"),
		reply: false,
		retweet: false,
	};
	
	let news_article = NewsArticle {
		headline: String::from("Breaking"),
		location: String::from("Rolla MO USA"),
		author: String::from("Wenqing Hu"),
		content: String::from("SPX lower than 4000")
	};
	
	tweet.summarize();
	
	notify1(&tweet);
	
	notify2(&news_article);
	
	notify3(&tweet, &news_article);
	
	notify4(&tweet, &news_article);
}


pub fn notify1(item: &impl Summary) {
	item.summarize();
}

pub fn notify2<T: Summary>(item: &T) {
	item.summarize();
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
	item1.summarize();
	item2.summarize();
}

pub fn notify4<T,U>(item1: &T, item2: &U) 
	where 	T: Summary, 
			U: Summary, {
	item1.summarize();
	item2.summarize();
}


fn lifetimes1<'a>() {
	let r: &'a i32;
	{
		let x: &'a i32 = &5;
		r = &x;
	}
	println!("r={}", r);
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	println!("longest");
	let z = "Wenqing";
	&z
}


fn lifetimes2() {
	let x = String::from("Hello");
	let y = String::from("world");
	longest(&x, &y);
}











