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

## 2. Primitives

### 2.1 Scalar Types

- Signed Integers `i8`,`i16`,`i32`,`i64`,`i128` and `isize` (pointer size)
- Unsigned Integers `u8`,`u16`,`u32`,`u64`,`u128` and `usize` (pointer size)
- Floating Points `f32` and `f64`
- Booleans `true` or `false`
- Char `a`,`α` or `∞` unicode 4 byte values.
- The unit type `()`, whose only possible value is an empty tuple: `()`
- Represented in [primitives.rs](src/primitives.rs)

### 2.2 Compound Types

- Arrays like `[1, 2, 3]`
- Tuples like `(1, true)`
- Represented in [primitives.rs](src/primitives.rs)

### 2.3 Literals And Operators

- Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true` and the unit type `()` can be expressed
  using literals.
- Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000, and 0.000_001
  is the same as 0.000001.
- Operators are similar to other C-like langs.
- Represented in [primitives.rs](src/primitives.rs)

### 2.4 Tuples

- Collection of values of different types.
- `tuple.index` can be used to get value.
- Represented in [primitives.rs](src/primitives.rs)

### 2.5 Arrays

- An array is a collection of objects of the same type `T`, stored in contiguous memory.
- Arrays are created using brackets `[]`.
- Length is known during compile time is part of their type signature `[T; length]`.
- Represented in [primitives.rs](src/primitives.rs)

## 3. Custom Types

### 3.1 Structs

- There are three types of structures ("structs") that can be created using the `struct` keyword.
- Tuple structs, which are, basically, named tuples.
- The classic C structs
- Unit structs, which are field-less, are useful for generics.
- Represented in [custom_types.rs](src/custom_types.rs)

### 3.2 Enums

- The `enum` keyword allows the creation of a type which may be one of a few different variants.
- Represented in [custom_types.rs](src/custom_types.rs)

### 3.3 Constants

- Rust has two different types of constants.
- `const`, An Unchangeable Value.
- `static`, A possibly mutable variable with `'static` lifetime.
- The static lifetime is inferred and does not have to be specified.
- Accessing or modifying a mutable static variable is `unsafe`.
- Represented in [custom_types.rs](src/custom_types.rs)

## 4. Variable Bindings

### 4.1 Mutability

- Rust provides type safety via static typing.
- Variable bindings can be type annotated when declared (or the compiler will do it through context)
- Values (like literals) can be bound to variables, using the `let` binding.
- Variable bindings are immutable by default, but this can be overridden using the `mut` modifier.
- Represented in [variable_bindings.rs](src/variable_bindings.rs)

### 4.2 Scope and Shadowing

- Variable bindings have a scope, and are constrained to live in a block.
- Shadowing is also allowed.
- Represented in [variable_bindings.rs](src/variable_bindings.rs)

### 4.3 Freezing

- When data is bound by the same name immutably, it also freezes.
- Frozen data can't be modified until the immutable binding goes out of scope
- Represented in [variable_bindings.rs](src/variable_bindings.rs)