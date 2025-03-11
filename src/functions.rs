#![allow(dead_code)]
pub fn main() {
    println!("Functions!");
    functions();
    expressions();
}

fn functions() {
    fn print_num(x: u32) {
        println!("The Num Is {}", x);
    }

    print_num(5);

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    print_labeled_measurement(5, 'h');
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    fn num_return(x: i32) -> i32 {
        let stringy = x.to_string() + "420";
        let num = stringy.parse().unwrap();
        num
    }

    let x = num_return(69);

    println!("The value of x is: {x}");
}
