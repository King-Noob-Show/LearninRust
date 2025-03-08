# Rust Learning Journey

## 1. Hello World!

### 1.1 The Infamous Hello World!

- To print in rust like other languages is simple
- `println!` is the macro for this.
- A rust file should have a `main()` to work.
- Printing Hello World is represented in [hello_world.rs](src/hello_world.rs)

### 1.2 Comments

- Comments in rust are just like JS or other languages
- `//` or `/* */` can be used to comment.
- For Example:-

```rust
fn main() {
    // This is a comment
    println!("Omg This Is Just Like Others!");
    /*
     * Multiple lines!
     * And this is a comment block!
     */
    println!("Hello, world!");
}
```

### 1.3 The Print Macro

- Printing has many macros.
- They're defined in `std::fmt`
- `format!` => write formatted text to String
- `print!` => same as `format!` but the text is printed to the console (io::stdout).
- `println!` => same as `print!` but a newline is appended.
- `eprint!` => same as `print!` but the text is printed to the standard error (io::stderr).
- `eprintln!` => same as `eprint!` but a newline is appended.
- The Print Macro should be represented in [print.rs](src/print.rs)

### 1.4 The Debug Trait

- All types which want to use std::fmt formatting traits require an implementation to be printable.
- Automatic implementations are only provided for types such as in the std library.
- The `fmt::Debug` trait makes this very easy.
- All types can `derive` (automatically create) the fmt::Debug implementation.
- This is not true for `fmt::Display` which must be manually implemented.
- The Debug Trait should also be represented in [print.rs](src/print.rs)

### 1.5 `fmt::Display` Integration

- It's much cleaner than `fmt::Debug`
- Represented in [print.rs](src/print.rs)

### 1.6 The Format Macro

- Represented in [print.rs](src/print.rs)