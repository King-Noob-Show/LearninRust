#![allow(dead_code)]

use std::fs::File;

pub fn main() {
    println!("Error Handling!");
    panik();
    recoverable();
}

fn panik() {
    // panic!("Crash And Burn!");
}

fn recoverable() {
    /*
     *
     * let greeting_file = File::open("hello.txt");
     *
     *
     * let greeting_file = match greeting_file {
     *  Ok(file) => file,
     *  Err(E) => panic!("File Not Found!\n{}", E),
     * };
     *
     */
}
