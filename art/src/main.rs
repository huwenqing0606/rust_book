extern crate art;

use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let Color = mix(red, yellow);
    println!("{:?}", Color);
}
