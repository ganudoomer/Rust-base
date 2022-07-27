fn main() {
    println!("Hello, world!");
    // These are how&& you declare variables
    let some_thing = "Hey";
    println!("{}", some_thing);
    // This is how you declare mutable variables
    let mut x = 32;
    println!("{}", x);
    x = 54;
    println!("{}", x);
    // This is how you declare constants
    // Cannot be changed in runtime
    // Has to remain constant
    const SOMETHING_THAT_IS_CONSTANT: &str = "Hey there";
    println!("{}", SOMETHING_THAT_IS_CONSTANT);

    // Two types of Data Types
    // Scalar and Compound
    // Scalar
    // Integers
    // Floating Point Numbers
    // Booleans
    // Char

    // Int
    // Signed and UnSinged
    // i32  both        u32 Only be positiv
    // Compound
    // Types that represent a group of values
    let tup = ("Hey", 12);
    let (name, number) = tup;
    println!("{} {}", name, number);
    println!("{}", tup.0);
    let error_codes = [12, 23, 45];
    let byte = [0; 8];
    print!("{}", byte[0]);
    println!("{}", error_codes[0]);
    let sum = my_function(12, 39);
    println!("{}", sum);
    // Control flow

    // loop
    let mut counter = 0;
    let sum: i32 = loop {
        counter += 1;
        if counter == 1 {
            break counter;
        }
    };
    println!("{}", sum);
    for element in error_codes.iter() {
        println!("{}", element);
    }
    for numb in 1..4 {
        println!("{}", numb);
    }
}

fn my_function(x: i32, y: i32) -> i32 {
    x + y
}
