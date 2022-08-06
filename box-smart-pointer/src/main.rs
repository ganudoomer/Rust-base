use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("Hello, world!,{} ", b);
    let list = Cons(32, Box::new(Cons(12, Box::new(Cons(12, Box::new(Nil))))));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("{}", name);
}
