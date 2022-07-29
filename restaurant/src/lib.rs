// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_wait_list();

//     front_of_house::hosting::add_to_wait_list();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::eat_at_restaurant();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peach"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     let soup = back_of_house::Appetizer::Salad;
//     let salad = back_of_house::Appetizer::Salad;
// }
mod front_of_house;
// use self::front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_something_to_the_list();
}
