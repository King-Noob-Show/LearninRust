#![allow(dead_code)]
pub fn main() {
    println!("Custom Types!");
    structs();
    enums();
    constants();
}

fn structs() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // A Unit Struct
    struct Unit;

    // A Tuple Struct
    struct Pair(i32, f32);

    // A normal struct
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    let point1: Point = Point { x: 4.2, y: 6.9 };
    let point2: Point = Point { x: 6.9, y: 4.2 };

    println!("Point coordinates 1: ({}, {})", point1.x, point1.y);
    println!("Point coordinates 2: ({}, {})", point2.x, point2.y);

    let bottom_right: Point = Point { x: 6.9, ..point2 };
    println!("bottom_right : ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point1;

    println!("Point1 point: ({}, {})", left_edge, top_edge);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    println!(
        "Rectangle point: ({:?}, {:?})",
        _rectangle.top_left, _rectangle.bottom_right
    );

    // Instantiate a unit struct
    let _unit = Unit;

    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn enums() {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and returns nothing.
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("Page Loaded"),
            WebEvent::PageUnload => println!("Page Unload"),
            WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
            WebEvent::Paste(s) => println!("Pasted '{}'.", s),
            WebEvent::Click { x, y } => {
                println!("Clicked at x = {}, y = {}", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('X');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("Ooga Booga".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // Type Aliases
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    type Toodaloo = VeryVerboseEnumOfThingsToDoWithNumbers;

    impl Toodaloo {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

fn constants() {
    static LANGUAGE: &str = "RUST";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
