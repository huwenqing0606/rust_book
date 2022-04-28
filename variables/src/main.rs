// basic variables and control flows

use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours is {} seconds", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;
    println!("x = {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // various other functions mentioned in this chapter
    fn_array(1);
    statements_expression();
    println!("{}", return_value(1));
    control_flow();
}

fn fn_array(x: i32) {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is : {}, add x is {}", 
        index, element, element+x
    );
}

fn statements_expression() {
    let y={
        let x = 3;
        x+1
    };
    println!("y={}",y);
}

fn return_value(x: u32)->u32{
    return x+1;
}

fn control_flow(){
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }    

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number-=1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

}