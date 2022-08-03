#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 {
            panic!("Guess a number that is below 100");
        }
        Guess { value }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn greetings(name: &str) -> String {
    format!("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // #[test]
    // fn larger_can_hold() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };

    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };
    //     assert!(!smaller.can_hold(&smaller));
    // }
    #[test]
    fn greeting_contains_name() {
        let greet = greetings("Hello");
        assert!(
            greet.contains("Hello"),
            "Greeting did not contain name {}",
            greet
        );
    }

    // #[test]
    // fn failing_test() {
    //     panic!("Make this test fail");
    // }
    #[test]
    #[should_panic]
    fn should_panic() {
        Guess::new(200);
    }
}
