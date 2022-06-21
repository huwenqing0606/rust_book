// enums example

enum IpAddr {
	V4(String),
	V6(String),
}


impl IpAddr {
	fn call(&self) -> &str {
		match self {   
			IpAddr::V4(string) => {string}
			IpAddr::V6(string) => {string}
		}
	}
}
			

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn ip_addr_print_v4(addr: IpAddr) -> bool {
    if let IpAddr::V4(string) = addr {
        println!("IpAddr (type V4) = {}", string);
        true
    }
    else {
        println!("IpAddr is not V4");
        false
    }
}


fn main() {
	let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

	println!("IpAddr: home is {}, IpAddr: loopback is {}", home.call(), loopback.call());

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five = {:?}, six = {:?}, none = {:?}", five, six, none);

    ip_addr_print_v4(home);
    ip_addr_print_v4(loopback);

    let a: Option<i32> = Some(1);
    let b: Option<i32> = Some(1);
    let c: Option<i32> = a.and_then(|a| b.map(|b| a + b));
    match c {
        None => println!("None"),
        Some(val) => println!("c value is {}", val),
    }
}