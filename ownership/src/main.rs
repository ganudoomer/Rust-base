fn main() {
    println!("Hello, world!");
    // Garbage Collector vs Manual model vs Ownership Model
    // Rust uses borrow checker, Checks everything related to memory allocation in Compilation

    // Ownership Rules
    // 1. Each value in rust has variable that's called it's owner
    // 2. There can be only one owner at a time
    // 3. When the owner goes out of scope, the value is dropped

    // Move(Not shallow copy)
    let s1 = String::from("Hello");
    // Here values is moved no copied
    let s3 = s1;
    //  we can't access s1

    // Here we are copying the values with clone method
    let s2 = s3.clone();
    // We can access all the values
    println!("{} {}", s3, s2);
    takes_ownership(s2);
    // After fn takes ownership of s2 we can't access the variable
    // println!(s2);

    // Here we get the ownership of the string
    let s5 = gives_ownership();
    println!("{}", s5);
    // Here we take a give back the ownership after a certain task
    let s6 = takes_and_gives_back_ownership(s5);
    println!("{}", s6);

    // References
    // Gives in a ref of the string
    // Ref are immutable by nature
    let length = calculate_length(&s6);
    println!("{}", length);
    // Mutable ref
    // We can only have one mutable ref at one scope
    // We cannot mix immutable ref with mutable ref

    let mut s = String::from("Hello");
    let r1 = &s;
    println!("{}", r1);
    let r2 = &mut s;
    //
    println!("{}", r2);

    // Ref Rules
    // The rules of references
    // 1. At any given time, you can have either one mutable ref or any number of immutable reference
    // This is to prevent Data Race
    // 2. References must be always be valid

    // Slices
    let mut s = String::from("Hello world");
    let hello = &s[0..7];
    let s1 = first_word(&s);
    println!("{}", s1);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    return length;
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn gives_ownership() -> String {
    // This function makes a string a returns the ownership
    let str = String::from("Hey there");
    return str;
}

fn takes_and_gives_back_ownership(str: String) -> String {
    str
}
