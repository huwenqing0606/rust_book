//! # Art
//!
//! A library for modeling artistic concepts

pub mod kinds {
    /// The primary colors according to the RYB color model

    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // .. snip ..
        SecondaryColor::Green
    }
}
