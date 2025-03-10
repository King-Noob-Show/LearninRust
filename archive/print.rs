#![allow(dead_code)]
use std::fmt::{self, Display, Formatter};

pub fn main() {
    printing();
    debug();
    display();
    formatting();
}

fn printing() {
    // The Normal Print
    print!("Hello, ");
    print!("you noobs.\n");

    // The Line Adding Print
    println!("I am pro.");
    println!("Okay maybe not");

    // In general, the `{}` will be automatically replaced with any arguments.
    println!("{} days", 31);

    // Positional Args With Integers
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Same with named args
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formats can also be used
    println!("{}", 69420);
    println!("Binary => {:b}", 69420);
    println!("Octal => {:o}", 69420);
    println!("Hex => {:x}", 69420);

    // Padding
    println!("{num:>10}", num = 69420);
    println!("{num:0>10}", num = 69420);
    println!("{num:0<10}", num = 69420);

    // Only types that implement fmt::Display can be formatted with `{}`.
    // Surrounding Vars Can Also Be Used

    let num: i32 = 10;
    let width: usize = 5;

    println!("{num:>width$}");
}

fn debug() {
    // This can't be printed with fmt::Display or fmt::Debug cuz no derive
    struct UnPrintable(i32);

    // The derive attribute automatically creates the implementation required to make this print with fmt::Debug
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // DebugPrintable is printable.
    println!("Now {:?} will print!", DebugPrintable(3));
    println!("Now {:?} will print!", Deep(DebugPrintable(7)));

    // Pretty Print with {:#?}
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Steve";
    let age = 27;
    let person = Person { name, age };

    println!("{:#?}", person)
}

fn display() {
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    fn smol_activity() {
        #[derive(Debug)]
        struct Complex {
            real: f64,
            imag: f64,
        }

        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} + {}i", self.real, self.imag)
            }
        }

        let complex = Complex {
            real: 3.3,
            imag: 7.2,
        };

        println!("Compare complex:");
        println!("Display: {}", complex);
        println!("Debug: {:?}", complex);
    }

    smol_activity();

    fn interesting() {
        struct List(Vec<i32>);

        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let vec = &self.0;
                write!(f, "[")?;

                for (count, v) in vec.iter().enumerate() {
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }

                write!(f, "]")
            }
        }

        let v = List(vec![1, 2, 3]);
        println!("{}", v);
    }

    interesting();
}

fn formatting() {
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` is like format!, but it will write the formatted string into a buffer (the first argument).
            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let r = self.red as u32 * 65536;
            let g = self.green as u32 * 256;
            let b = self.blue as u32;
            let colour_final = r + g + b;
            let hex = format!("{:X}", colour_final);
            let formatted_hex = String::from("0x") + &hex;
            write!(
                f,
                "RGB: ({}, {}, {}), {:0<8}",
                self.red, self.green, self.blue, formatted_hex
            )
        }
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{}", color);
    }
}
