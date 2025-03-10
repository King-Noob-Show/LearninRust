use std::mem;
pub fn main() {
    println!("Primitives!");
    scalars();
    compounds();
    operators();
    tupler();
    arrays();
}

fn scalars() {
    // Vars can be type annotated
    let boolean: bool = true;
    let num1: i32 = 69;
    let num2: f32 = 42.42;

    // Suffix Annotation
    let num3 = 420i32;
    let num4 = 42.3f32;

    // If not annotated, default types i32 or f64 will be used
    let num5 = 69420; // i32
    let num6 = 69.420; // f64

    // Can also be inferred from context

    let mut num7 = 420; // Changed to i64 now
    println!("{}", num7);
    num7 = 420000000000i64;
    println!("{}", num7);

    // Value type can be changed by overshadowing

    let num7 = false;

    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}",
        boolean, num1, num2, num3, num4, num5, num6, num7
    )
}

fn compounds() {
    // Array signature consists of Type T and length as [T; length].
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types.
    let tup = (500i32, 6.4f32, 1i32, false, -5.04f32);

    println!("{}\n{}\n{}\n", arr[0], tup.0, tup.1);
}

fn operators() {
    // Addition
    println!("{} + {} = {}", 2, 3, 2 + 3);

    // Subtraction
    println!("{} - {} = {}", 3, 2, 3 - 2);

    // Scientific Notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Interesting Bool Logic
    println!("True AND False is {}", false); // true && false
    println!("True OR False is {}", true); // true || false
    println!("NOT True is {}", !true);
    println!("NOT False is {}", !false);

    // Bitwise operations (Very Weird?)
    /*
     * println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
     * println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
     * println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
     * println!("1 << 5 is {}", 1u32 << 5);
     * println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
     */

    // Underscores for better readability.
    println!("One Million Is Written As:- {}", 1_000_000i32);
}

fn tupler() {
    // A tuple with a bunch of different types.
    // u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, bool
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    println!("tuple of tuples: {:?}", tuple_of_tuples.1.0);

    // More than 12 elements can't be printed
    // Tuples can be used as function arguments and as return values.
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // `let` can be used to bind the members of a tuple to variables.
        let (int_param, bool_param) = pair;

        (bool_param, int_param)
    }

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}

fn arrays() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // This function borrows a slice.
    fn analyze_slice(slice: &[i32]) {
        println!("First element of the slice: {}", slice[0]);
        println!("The slice has {} elements", slice.len());
    }

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Empty Slice
    /*
     * let empty_array: [u32; 0] = [];
     * assert_eq!(&empty_array, &[]);
     * assert_eq!(&empty_array, &[][..]); // Same but more verbose
     */

    // Arrays can be safely accessed using `.get`.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
