#![allow(dead_code)]

use std::collections::HashMap;

pub fn main() {
    println!("Common Collections!");
    collections();
    stringy();
    hashmaps();
}

fn collections() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4, 5];

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);

    let third: &i32 = &v2[2];

    println!("The third element of v2 is {}", third);

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element of v1 is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    println!("The first element is: {first}");

    v.push(6);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}

fn stringy() {
    let _s = String::new();

    let data = "Initial Contents";

    let _s = data.to_string();

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שלום");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 = {}, s2 = {}", &s1, &s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("s2 = {}, s3 = {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{}", s3);

    println!("{s}");
    println!("{}, {}, {}", s1, s2, s3);

    let s1 = String::from("Sigma");

    for i in s1.chars() {
        println!("{}", i);
    }

    for i in s1.bytes() {
        println!("{}", i);
    }
}

fn hashmaps() {
    let mut x = HashMap::new();

    let s = String::from("Hello");

    x.insert(String::from("Ez Noobs"), 0);
    x.insert(String::from("Ooga Booga"), 1);
    x.insert(s, 2);

    println!("{:#?}", x);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score is {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:#?}");
}
