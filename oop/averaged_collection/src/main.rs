use averaged_collection::AveragedCollection;

fn main() {
    println!("Object-Oriented Programming in Rust: Examples");

    let mut ac = AveragedCollection::new(vec![], 0.0);
    ac.add(1);
    ac.add(2);
    println!("{:?}, average = {}", ac, ac.average());
    ac.remove();
    println!("{:?}, average = {}", ac, ac.average());
    ac.remove();
    println!("{:?}, average = {}", ac, ac.average());
}
