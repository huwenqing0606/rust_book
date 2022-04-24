// example code for error handing
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //use_panic();
	//use_backtrace();
	use_Result();
}


fn use_panic() {
	panic!("crach and burn");
}


fn use_backtrace() {
	let v = vec![1, 2, 4];
	println!("{}", v[99]);
}


fn use_Result() {
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