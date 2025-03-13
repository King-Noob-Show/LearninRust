#![allow(dead_code)]
#![allow(unused_variables)]
pub fn main() {
    println!("Enums!");
    enums();
    match_control_flow();
    if_let();
}

fn enums() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("Calling");
        }
    }

    let message = Message::Write(String::from("Wassup Noobs"));
    message.call();

    let some_number = Some(32);
    let some_string = Some("Ooga Booga");

    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
}

fn match_control_flow() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter;
    let coin3 = Coin::Nickel;
    let coin4 = Coin::Dime;

    println!(
        "Value In Cents: {}, {}, {}, {}",
        value_in_cents(coin1),
        value_in_cents(coin2),
        value_in_cents(coin3),
        value_in_cents(coin4)
    );

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!(
        "Five is {:?}, Six is {:?} and None is {:?}",
        five, six, none
    );

    let dice_roll: u8 = 7;

    fn add_fancy_hat() {
        println!("Added Fancy Hat!");
    }

    fn remove_fancy_hat() {
        println!("Removed Fancy Hat!");
    }

    fn move_player(num_spaces: &u8) {
        println!("Moved Player {} Spaces!", num_spaces);
    }

    match dice_roll {
        3 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        other => move_player(&other),
    }
}

fn if_let() {
    let config_max: Option<u8> = Some(3);

    match config_max {
        Some(max) => println!("The maximum number is {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum number is {}", max);
    }
}
