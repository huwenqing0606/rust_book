// enums example

enum IpAddr {
	V4(String),
	V6(String),
}


impl IpAddr {
	fn call(&self) -> &str {
		match self {   
			IpAddr::V4(string) => {println!("V4 = {}", string); "V4"}
			IpAddr::V6(string) => {println!("V6 = {}", string); "V6"}
		}
	}
}
			
			
fn main() {
	let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

	
	println!("home is {}", home.call());
	println!("loopback is {}", loopback.call());

}