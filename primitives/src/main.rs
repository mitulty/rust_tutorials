/* @Author: Mitul
   @Description: Primitives Data Types in Rust
*/
/*
****************************************************************{Data Types}**************************************************************************************
- Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. Rust is a
  statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type to use based on
  the value and how it is used. In cases when many types are possible, a type annotation is added.


- Rust provides access to a wide variety of primitives. A sample includes:
- Scalar Types
   - Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
   - Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
   - Floating point: f32, f64
   - char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
   - bool either true or false
   - The unit type (), whose only possible value is an empty tuple: (). Despite the value of a unit type being a tuple, it is not considered a compound type
     because it does not contain multiple values.s
- Compound Types
   - Arrays like [1, 2, 3]
   - Tuples like (1, true)
- Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64.
- A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.
- An integer is a number without a fractional component. The built in integer types are: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize and usize.
  Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative—
  in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a
  sign (unsigned). Signed numbers are stored using two’s complement representation. Each signed variant can store numbers from {-(2^n - 1) to (2^(n - 1) - 1)}
  inclusive, where n is the number of bits that variant uses. Unsigned variants can store numbers from {0 to 2^n - 1}. Additionally, the isize and usize types
  depend on the architecture of the computer on which the program is running on. Integer type defaults to i32. The primary situation in which code can use
  isize or usize is when indexing some sort of collection.
- Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: 0x, 0o or 0b.
- Number literals can be written in different forms: 98_222, 0xfff, 0o77, 0b1111_0000 and b'A' (Byte- u8 only). Number literals use _ as a visual separator to make
  the number easier to read. The number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type.
- When compiling in debug mode, Rust includes checks for integer overflow that can cause program to panic at runtime if this behavior occurs. When compiling
  in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs
  two’s complement wrapping. In short, values greater than the maximum value the type can hold "wrap around" to the minimum of the values the type can hold.
- Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are
  32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more
  precision. All floating-point types are signed. Rust also supports scientific E-notation, e.g. 1e6, 7.6e-4. The associated type is f64.
- Rust supports the basic mathematical operations for all the number types: addition, subtraction, multiplication, division, and remainder. Integer division
  truncates toward zero to the nearest integer. The operators available and their precedence in Rust are similar to other C-like languages.
- A Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool. The main
  way to use Boolean values is through conditionals, such as an if expression.
- Rust’s char type is the language’s most primitive alphabetic type. The char literals are specified with single quotes, as opposed to string literals, which
  use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
  Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from
  U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.


- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
- A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared,
  they cannot grow or shrink in size. A tuple is created by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type,
  and the types of the different values in the tuple don’t have to be the same. Optional type annotations can be added.
- A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature
  (T1, T2, ...), where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.
- Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in
  some other languages, arrays in Rust have a fixed length. Arrays are useful when data is to be allocated on the stack rather than the heap or when the number
  of elements is fixed. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed
  to grow or shrink in size.
- An array’s type can be written using square brackets with the type of each element, a semicolon, and then the number of elements in the array. An array can be
  initialized to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square
  brackets.
- An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. To access elements of an array 0-based indexing is used.
- If invalid location is accessed then the Rust protects against this by immediately exiting instead of allowing the memory access and continuing.
- Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object; the first word is a pointer to the data,
  the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64.
- Slices can be used to borrow a section of an array and have the type signature &[T].
*/

use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[allow(unused_variables)]
fn main() {
    //---------------------------------------------------------------------------------------
    // Data Types
    let _guess: u32 = "42".parse().expect("Not a number!"); // Type Annotation
    let _x = 2.0; // f64

    let _a: i32 = 98_222; // Decimal
    let _b: i32 = 0xff; // Hex
    let _c: i32 = 0o77; // Octal
    let _d: i32 = 0b1111_0000; // Binary
    let _e: u8 = b'A'; // Byte (u8 only)
    let _f: u8 = 255;

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let _y: f32 = 3.0; // f32
    let _three_hours: u32 = THREE_HOURS_IN_SECONDS;

    // Mathematical Operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Boolean Type
    let _t = true;

    let _f: bool = false; // with explicit type annotation
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context.
    let mut _inferred_type = 12; // Type i64 is inferred from another line.
    _inferred_type = 4294967296i64;

    //---------------------------------------------------------------------------------------
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Integer division
    println!("5 / 2 = {}", 5 / 2);

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    //---------------------------------------------------------------------------------------
    /* Compound types - Array and Tuple */
    
    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);

    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    //This first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z.
    //this is called destructuring because it breaks the single tuple into three parts.
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of (x,y,z) is: {}{}{}", x, y, z);
    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    // But long Tuples (more than 12 elements) cannot be printed.
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
  
    //---------------------------------------------------------------------------------------
    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];


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

    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr = [3; 5];
    let _first = a[0];
    let _second = a[1];

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let _ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    // TODO: Resolve the dependency erro.
    // println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}
