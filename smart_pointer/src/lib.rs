pub mod my_list {
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }
}


pub mod mp3{
    use std::ops::Deref;

    pub struct Mp3 {
        pub audio: Vec<u8>,
        pub artist: Option<String>,
        pub title: Option<String>,
    }

    impl Deref for Mp3 {
        type Target = Vec<u8>;

        fn deref(&self) -> &Vec<u8> {
            &self.audio
        }
    }
}

pub mod custom_smart_pointer {
    pub struct CustomSmartPointer {
        pub data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer!");
        }
    }
}