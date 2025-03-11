#![allow(dead_code)]
pub fn main() {
    println!("Control Flow!");
    if_else();
    loops();
}

fn if_else() {
    let number = 4; // 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number was something other than zero");
    }

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loops() {
    let mut counter = 69;
    let mut number_of_loops = 0;

    let result = loop {
        counter += 1;
        number_of_loops += 1;

        if counter == 690 {
            println!("No. of loops: {}", number_of_loops);
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    /*
    let mut number = 10;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
     */
    
    for number in (1..11).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
    
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("The value is: {}", element);
    }
}
