pub fn main() {
    println!(
        "The first letter of the alphabet is {} and the last one is {}",
        "A", "B"
    );
    create_vars();
    add(69, 420);
    chars();
    tuples_omg();
    structs_no_way();
    enums_omg();
    functions_omg(55, 57);

    // Order Three Cars.
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );
}

fn create_vars() {
    let z = 5;
    let a_word = "A word";
    let y;
    y = 138;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num + 5;
    println!(
        "The vars in order are {}, {}, {}, {}, {}",
        z, a_word, y, x, shadow_num
    );
}

fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
    println!("Different Way With Type Idk, {}", 6i32 + 7i32);
    println!("Floating, {}", 7.037f64 + 4835.12456f64);
    let is_bigger: bool = x < y;
    println!("The statement {} is less than {} is {}", x, y, is_bigger);
    let emoji = "😃";
    println!("{emoji} and {x}")
}

fn chars() {
    let char_1: char = 'a';
    let char_2: char = 'B';
    let emoji = '😃';
    let stringing: &str = "This is a normal string";
    let stringing_part2 = stringing.to_ascii_uppercase(); // This is a String now interesting.
    println!("Everything is {char_1} {char_2} {emoji} {stringing} {stringing_part2}")
}

fn tuples_omg() {
    let tuple_e = ('E', 5i32, true); // char, i32, bool
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );
}

fn structs_no_way() {
    struct Student {
        name: String,
        level: u8,
        is_pro: bool,
    }
    // No Semicolon?
    struct Grades(char, char, char, char, f32);
    // struct Unit;

    let user_1 = Student {
        name: String::from("x"),
        level: 1,
        is_pro: true,
    };

    let _mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!(
        "{} is level {}. He is pro? {}",
        user_1.name, user_1.level, user_1.is_pro
    );
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.is_pro, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );
}

fn enums_omg() {
    #[derive(Debug)]
    struct KeyPress(String, char);
    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(MouseClick { x: 0, y: 0 });

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    let we_key = WebEvent::WEKeys(keys);

    println!(
        "Web Event Enum Structure:- {:#?}, {:#?}, {:#?}",
        we_load, we_key, we_click
    )
}

fn functions_omg(x: i32, y: i32) {
    if x == 0 || y == 0 {
        return;
    }

    println!("The value of their sum is {}", x + y);
}

// Function to construct a car, my first exercise.
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color,
        transmission,
        convertible,
        mileage: 0,
    }
}
