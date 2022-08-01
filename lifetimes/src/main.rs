use std::fmt::Display;

fn main() {
    // Borrow checker checks for ref in compile time

    // Does not work because x lifetime ends after the scope
    // let r: i32;
    // {
    //     let x = 5;
    //     let r = &x;
    // }
    // println!("Hello, world! {}", r);

    let string1 = String::from("Hey there");
    let string2 = String::from("Hey three afsdf");

    let result = longest(&string1, &string2);
    println!("{}", result);

    let res = longest_with_an_announcement(&string1, &string2, "Hey");
    println!("{}", res);

    // This also does not work because
    // We use a ref of variable that the lifetime that has ended

    // let string4 = String::from("Hey three afsdf");
    // let result;
    // {
    //     let string3 = String::from("Hey there");
    //     result = longest(&string3, &string4);
    // }
    // println!("{}", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// we don't which value is going to live
// This can be only determined on runtime
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
