use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    let second = &v[1];

    match v.get(30) {
        Some(thrid) => println!("The third element is {}", thrid),
        None => print!("Hey"),
    }

    println!("The second variable is {}", second);

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
    println!("Hello, world!");

    // Strings
    // In rust strings are stored as a collections of UTF-8 encoded bytes
    let mut s = String::from("Hey there");
    s.push_str(" Hey");
    s.push('!');
    println!("{}", s);

    let s1 = String::from("Hello ");
    let s2 = String::from("World");

    let s3 = s1 + &s2;

    println!("{}", s3);
    // Three ways we can represent bytes in unicode
    // Bytes
    // Scalar Values
    // Grapheme clusters
    let hello = String::from("Hello");

    for c in hello.graphemes(true) {
        println!("{}", c)
    }
    // Hash Map
    let mut scores = HashMap::new();
    scores.insert("key", "home");
    scores.insert("hey", "there");
    let hey = scores.get("hey");
    match hey {
        Some(hello) => println!("{}", hello),
        None => println!("Hey"),
    }
    scores.entry("key").or_insert("There");
    scores.entry("keys").or_insert("There");

    for (key, value) in &scores {
        println!("key {}  value {}", key, value)
    }
    let text = "Hello world world hello";
    let mut hash = HashMap::new();

    for word in text.split_whitespace() {
        let count = hash.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", hash);

    // fn a() {
    //     b();
    // }

    // fn b() {
    //     c(22)
    // }

    // fn c(number: i32) {
    //     if number == 22 {
    //         panic!("Crash and burn");
    //     }
    // }
    // a();

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error)
            }
        },
    };

    // Error propagation
    // Unwrap except
}
