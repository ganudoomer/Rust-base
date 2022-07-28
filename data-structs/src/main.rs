struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    //Associate functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut user = User {
        username: String::from("hey there"),
        active: true,
        email: String::from("some@gmail.com"),
        sign_in_count: 1,
    };

    let name = user.username;
    user.email = String::from("jey@g.com");

    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 240,
    };

    let rect3 = Rectangle::square(300);
    println!("{:?}", rect3);
    println!("{:?}", rect.can_hold(&rect2));

    // ENUMS
    let four = IpAddressKind::IPV4;
    let six = IpAddressKind::IPV6;
    // ENUMS WITH STRING
    let four_str = IpAddressWithString::IPV4(String::from("127.0.0.1"));
    let five_u8 = IpAddressWithString::IPV6(129, 11, 1);
    let message = IpAddressWithString::MESSAGE { x: 8 };

    // OPTION ENUM
    let some_number = Some(6);
    let some_int: Option<u8> = None;

    let sum = 10 + some_number.unwrap_or(0);
    println!("{}", sum);
    // Pass a Enum inside an enum
    value_in_cents(Coin::Nickel(State::Alabama));
    // Using optional 
    let five = Some(8);
    let ans = plus_one(five);
    // If let 
    let some_value = Some(3);
    if let Some(3) = some_value {
        print!("Three");
    }
}

enum IpAddressKind {
    IPV4,
    IPV6,
}

enum IpAddressWithString {
    IPV4(String),
    IPV6(u8, u8, u8),
    MESSAGE { x: u8 },
}

impl IpAddressWithString {
    fn print() {
        println!("Hey there");
    }
}

enum State {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel(State),
    Dime,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 10,
        Coin::Nickel(state){
            println!("{}",state);
            return 9; 
        },
        Coin::Dime => 20,
    }
}


fn plus_one(x: Option<u8>) -> Option<u8> {
    match  x {
        Some(i) =>  Some(i+1),
        None => None,
    }
} 