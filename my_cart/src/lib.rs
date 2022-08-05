// //! # My Crate
// //!
// //! `my_carte` is a collection of some functions that help you do something
// //! calculations are more convenient
// //!

// /// Add one number to a give number
// /// #Examples
// /// ```
// /// let arg = 5;
// /// let ans = my_cart::add_one(arg);
// /// assert_eq!(ans,6);q
// /// ```
// ///

// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    pub fn mix(p1: PrimaryColor, p2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
