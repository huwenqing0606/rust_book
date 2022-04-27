// example code for error handing
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;


fn main() {
    //use_panic();
	//use_backtrace();
	//use_Result_unwrap();
	//use_Result_expect();
	//read_username_from_file();
	//read_username_from_file_short();
	//read_username_from_file_short_short();
	let result = last_char_of_first_line("");
	match result {
		Some(c) => println!("result is {}", c),
		None => println!("result not found!"),
	};
	IpAddr_err();
	make_guess();
}


fn use_panic() {
	panic!("crach and burn");
}


fn use_backtrace() {
	let v = vec![1, 2, 4];
	println!("{}", v[99]);
}


fn use_Result_match() {
	let f = File::open("hello.txt");
	
	match f 
	{
		Ok(file) => {println!("file found!"); let f = file;},
		Err(error) => match error.kind() 
			{	ErrorKind::NotFound => match File::create("hello.txt") 
					{	Ok(fc) => {println!("file created"); let f = fc;},
						Err(e) => panic!("problem creating the file! {:?}", e),
					},
					
				other_error => panic!("problem opening the file! {:?}", other_error)
			}
	};
}


fn use_Result_unwrap() {
	let f = File::open("hello.txt").unwrap();
}


fn use_Result_expect() {
	let f = File::open("hello.txt").expect("Failed to open hello.txt");
}


fn read_username_from_file() -> Result<String, io::Error> {
	let f = File::open("hello.txt");
	
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};
	
	let mut s = String::new();
	
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}


fn read_username_from_file_short() -> Result<String, io::Error> {
	let mut s = String::new();
	
	File::open("hello.txt")?.read_to_string(&mut s)?;
	Ok(s)
}


fn read_username_from_file_short_short() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}


fn last_char_of_first_line(text: &str) -> Option<char> {
	text.lines().next()?.chars().next()
}


fn IpAddr_err() {
	use std::net::IpAddr;
	let home: IpAddr = "127.0.0.1".parse().unwrap();
	println!("home IpAddr = {:?}", home);
}


pub struct Guess {
	value: i32,
}


impl Guess {
	
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}", value);
		}
		Guess {value}
	}
	
	pub fn value(&self) -> i32 {
		self.value
	}
}


fn make_guess() {
	let guess1 = Guess::new(99);
	println!("guessed value 1 is {}", guess1.value());
	
	let guess2 = Guess::new(101);
	println!("guessed value 2 is {}", guess2.value());
}