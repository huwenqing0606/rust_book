// examples of ownership

fn main() {
    let mut s = "hello, ".to_owned();
    concat_literal(&mut s);
    println!("{}", s);

    clone_and_copy();

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    let mut s = String::from("hello");
    change(&mut s);
    println!("string = {}", s);

    let r1 = &s;
    let r2 = &s;
    println!("r1 = {} and r2 = {}", r1, r2);
    let r3 = &mut s;
    r3.push_str(", nice");
    println!("r3 = {}", r3);
    println!("s = {}", s);

    //let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
    println!("reference_to_something is {}", reference_to_something);

    let s = String::from("hello world!");
    let word = first_word(&s);
    println!("the first word is {}", word);

    string();
}

fn concat_literal(s: &mut String) {
    s.extend("world".chars());
}

fn clone_and_copy() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}


fn gives_ownership() -> String{
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    println!("word is {}", word);
    let word = first_word(&my_string[..]);
    println!("word is {}", word);
    let word = first_word(&my_string);
    println!("word is {}", word);

    let my_string_literal = "hello_world";
    let word = first_word(&my_string_literal[0..6]);
    println!("word is {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("word is {}", word);
    let word = first_word(my_string_literal);
    println!("word is {}", word);
}