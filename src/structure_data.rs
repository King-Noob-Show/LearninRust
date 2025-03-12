#![allow(dead_code)]
#![allow(unused_variables)]
pub fn main() {
    println!("Structures And Related Data!");
    structs();
    method_syntax();
}

fn structs() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1: User = User {
        username: String::from("King"),
        email: String::from("kingispro@noob.com"),
        sign_in_count: 59,
        active: true,
    };

    user1.sign_in_count += 1;

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true,
        }
    }

    let x = build_user(
        String::from("kingispro69@noob.com"),
        String::from("kingispro69"),
    );

    let user2 = User { ..user1 }; // Can't use user1 now

    let user3 = User {
        email: x.email,
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 69,
        height: 420,
    };

    fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }

    println!("The area of the rectangle is {}", area(&rect));
}

fn method_syntax() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn height(&self) -> bool {
            self.height > 0
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1: Rectangle = Rectangle {
        width: 69,
        height: 69, // 0,
    };

    if rect1.height() && rect1.width() {
        println!("The area of the rectangle is {}", rect1.area());
    } else {
        println!("Either the height or width is 0!");
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 400,
    };

    println!("Can rect1 hold rect2? -> {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? -> {}", rect1.can_hold(&rect3));


    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    
    let rect1 = Rectangle::square(69);
    
    println!("rect1: {}, {}", rect1.width, rect1.height)
}
