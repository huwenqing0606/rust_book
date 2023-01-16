use smart_pointer::my_list::List::{Cons, Nil};
use smart_pointer::mp3::Mp3;
use smart_pointer::custom_smart_pointer::CustomSmartPointer;
//use std::mem::drop;

fn main() {
    boxnew();
    let _list = Cons(1, Box::new(Cons(2, 
                            Box::new(Cons(3, 
                                Box::new(Nil))))));
    
    let my_favorite_song = Mp3 {
        audio: vec![1,2,3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells like Teen Spirit")),
    };

    assert_eq!(vec![1,2,3], *my_favorite_song);
    println!("{:?}", *my_favorite_song);

    compress_mp3(&my_favorite_song);

    let _c = CustomSmartPointer{data: String::from("some data")};

    println!("CustomSmartPointer created.");
    //drop(c);
    println!("Wait for it...");

}

fn boxnew() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    (&audio).to_vec()
}