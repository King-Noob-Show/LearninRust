#![allow(dead_code)]
pub fn main() {
    println!("Variables!");
    variables();
}

fn variables() {
    let x = 5; // Unchangeable
    println!("The value of x is {}", x);

    let mut x = 69; // Shadow + Changeable
    println!("The value of x is {}", x);
    x = 69420;
    println!("The value of x is {}", x);
    x = 20;
    println!("The value of x is {}", x);

    const POINTS: i64 = 10000000000;
    println!("Points: {}", POINTS);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
