use std::{thread, time::Duration};
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculated: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculated: T) -> Cacher<T> {
        Cacher {
            calculated,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculated)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating some stuff");
        // thread::sleep(Duration::from_secs(2));
        num
    });
    iterators();
    println!("{}", expensive_closure.value(10));
    println!("{}", expensive_closure.value(10));
}

fn iterators() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iterators = v1.iter();

    for item in v1_iterators {
        print!("{}", item);
    }
}

#[test]
fn iterators_demo() {
    let v1 = vec![1, 2, 3, 4];
    let mut v1_iterators = v1.iter();

    assert_eq!(v1_iterators.next(), Some(&1));
}

#[test]
fn iterators_sum() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iterators = v1.iter();
    let total = v1_iterators.sum();

    assert_eq!(10, total);
}

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
