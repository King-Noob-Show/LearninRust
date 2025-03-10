pub fn main() {
    println!("Variable Bindings!");
    mutability();
    scoping_shadows();
    freezing();
}

fn mutability() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32; // Put underscore to ignore warning.

    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
}

fn scoping_shadows() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding shadows the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn freezing() {
    let mut mutable_integer = 7i32;
    
    println!("The mutable integer is {}", mutable_integer);

    {
        // Shadowing by immutable mutable_integer
        let mutable_integer = mutable_integer;
        println!("The (im)mutable integer inside is {}", mutable_integer);

        // mutable_integer is frozen in this scope

        // mutable_integer goes out of scope
    }

    // Ok! `mutable_integer` is not frozen in this scope
    mutable_integer = 3;
    println!("New mutable integer: {}", mutable_integer);
}
