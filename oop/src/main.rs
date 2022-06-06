extern crate averaged_collection;
use averaged_collection::AveragedCollection;

fn main() {
    let mut ac = AveragedCollection::new(vec![], 0.0);
    ac.add(1);
    println!("{:?}", ac);
    println!("Object-Oriented Programming in Rust: Examples");
}
