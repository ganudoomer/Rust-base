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
}
