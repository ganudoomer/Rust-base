fn main() {
    // print! macro
    // Literal  String and Literal Int
    print!("Hello, world! \n");
    print!("{}, {}", "Hello", "World");
    print!("My number {}", 40);
    print!("\n My number: {}", 000140);
    print!("\n The sum is {} . \n", 80 + 34);
    print!("{}", 2.7 + 1.);
    //Will fail to compile
    //print!("{}, !", "Hello", "world");
    //print!("{}, {}!", "Hello");
    println!(
        "{}",
        "These
    are
    three lines"
    );
}
