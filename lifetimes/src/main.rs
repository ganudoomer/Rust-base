fn main() {
    // Borrow checker checks for ref in compile time

    // Does not work because x lifetime ends after the scope
    // let r: i32;
    // {
    //     let x = 5;
    //     let r = &x;
    // }
    // println!("Hello, world! {}", r);

    // let string1 = String::from("Hey there");
    // let string2 = String::from("Hey three");

    // let result = longest(&string1, &string2);
    // println!("{}", result);
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
