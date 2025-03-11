#![allow(dead_code)]
pub fn main() {
    println!("Data Types!");
    data_types();
}

fn data_types() {
    let num1: u32 = 1;
    let num2: i32 = -2;

    let num3: f32 = 69.0;
    let num4: f64 = -2.0;

    let sum = 5 + 10;

    let difference: f32 = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        num1, num2, num3, num4, sum, difference, product, quotient, truncated, remainder
    );

    let is_pro = true;
    let is_noob: bool = false;

    println!("is_pro is {}, and is_noob is {}", is_pro, is_noob);

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Chars:- {}, {} and {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {}, and tup {:?}", x, y, z, tup);
    println!("Another Method: {}, {} and {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let b: [i32; 4] = [1, 2, 3, 4];
    let c = [69; 10];

    println!(
        "Arrays:- {:?}\n{:?}\n{:?}\n{:?}\n{}",
        a, b, months, c, months[0]
    )
}
