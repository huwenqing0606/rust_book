// use lib.rs to manage crate tree
mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
		fn seat_at_table() {}
	}
	
	mod serving {
		fn take_order() {}
		fn serve_order() {}
		fn take_payment() {}
	}
}

mod back_of_house {
	
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
		
	}
	
	pub enum Appetizer {
		Soup,
		Salad,
	}
	
	impl Breakfast{
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}
	
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}
	
	fn cook_order() {}
}

use crate::front_of_house::hosting as front_hosting;

pub fn eat_at_restaurant() {
	// absolute path
	crate::front_of_house::hosting::add_to_waitlist();
	// relative path
	front_of_house::hosting::add_to_waitlist();
	
	// use
	// idiomatic way of writing
	front_hosting::add_to_waitlist();
	
	// order a breakfast in the summer with Rye toast
	let mut meal = back_of_house::Breakfast::summer("Rye");
	// change our mind about what bread we would like to have
	meal.toast = String::from("Wheat");
	println!("I'd lile {} toast", meal.toast);
	//meal.seasonal_fruit = String::from("blueberries"); //cannot compile due to privacy in mean.seasonal_fruit
	
	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}


use rand::Rng;

use std::collections::HashMap;
