#![allow(dead_code)]
pub fn main() {
    println!("Ownership!");
    ownership();
    references();
    slice_type();
}

fn ownership() {
    let mut s = String::from("Hello Noobs!");

    s.push_str(" I am King!");

    println!("{}", s);

    let s1 = String::from("Hello Noobs Part 2!");
    let s2 = s1; // Moved

    println!("{}", s2);

    let s1 = String::from("Hello!");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("Hello!!!!!!!!!!!!!!!");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }

    let s1 = gives_ownership();

    let s2 = String::from("So Pro Omg");

    let s3 = takes_and_gives_back(s2);

    fn gives_ownership() -> String {
        let some_string = String::from("I am Sigma");

        some_string
    }
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    println!("{}, {}", s1, s3);

    fn calculate_length(s: String) -> (String, usize) {
        let len = s.len();

        (s, len)
    }

    let s1 = String::from("I am so long haha!");

    let (s2, len) = calculate_length(s1);

    println!("{}, {}", s2, len);
}

fn references() {
    let s1 = String::from("Hello Noobs Part 3!");

    fn calc_len(s: &String) -> usize {
        s.len()
    }

    let s2 = calc_len(&s1);

    println!("{}", s2);
    println!("{}", s1);

    let mut s1 = String::from("Hello, ");

    fn add_string(s: &mut String) {
        s.push_str("World!");
    }

    add_string(&mut s1);

    println!("{}", s1);

    let mut s = String::from("Ooga Booga");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2}");

    let r3 = &mut s;

    println!("{r3}");
}

fn slice_type() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("Hello Noobs Part 4!");

    let word = first_word(&s);

    s.clear();

    println!("{}", word);

    let s = String::from("Hello Noobs Part 5!");

    let slice = &s[1..5];

    println!("{}", slice);

    let slice = &s[5..s.len()];

    println!("{}", slice);

    fn first_world_part2(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    let my_string = String::from("Hello Noobs Part 6!");

    let word = first_world_part2(&my_string);

    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
