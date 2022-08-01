fn main() {
    println!("Hello, world!");
    let chars = vec!["y", "d", "d", "b"];
    let largest = get_largest(chars);
    println!("{}", largest);
    // Structs also work with generics
}
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut larges = number_list[0];
    for number in number_list {
        if number > larges {
            larges = number;
        }
    }
    larges
}
