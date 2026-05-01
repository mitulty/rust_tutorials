/* @Author: Mitul
   @Description: Keywords and Variables in Rust.
*/
/*
****************************************************************{Keywords}**************************************************************************************
- Keywords: The Rust language has a set of keywords that are reserved for the use by the language only, much as in other languages. These words can not be used as
  names of variables or functions. Most of the keywords have special meanings and will be used to do various tasks in the Rust programs; a few have no current
  functionality associated with them but have been reserved for functionality that might be added to Rust in the future.
- The following is a list of keywords currently in use, with their functionality described.
    as - perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements
    async - return a Future instead of blocking the current thread
    await - suspend execution until the result of a Future is ready
    break - exit a loop immediately
    const - define constant items or constant raw pointers
    continue - continue to the next loop iteration
    crate - in a module path, refers to the crate root
    dyn - dynamic dispatch to a trait object
    else - fallback for if and if let control flow constructs
    enum - define an enumeration
    extern - link an external function or variable
    false - Boolean false literal
    fn - define a function or the function pointer type
    for - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
    if - branch based on the result of a conditional expression
    impl - implement inherent or trait functionality
    in - part of for loop syntax
    let - bind a variable
    loop - loop unconditionally
    match - match a value to patterns
    mod - define a module
    move - make a closure take ownership of all its captures
    mut - denote mutability in references, raw pointers, or pattern bindings
    pub - denote public visibility in struct fields, impl blocks, or modules
    ref - bind by reference
    return - return from function
    Self - a type alias for the type we are defining or implementing
    self - method subject or current module
    static - global variable or lifetime lasting the entire program execution
    struct - define a structure
    super - parent module of the current module
    trait - define a trait
    true - Boolean true literal
    type - define a type alias or associated type
    union - define a union; is only a keyword when used in a union declaration
    unsafe - denote unsafe code, functions, traits, or implementations
    use - bring symbols into scope
    where - denote clauses that constrain a type
    while - loop conditionally based on the result of an expression
- Keywords Reserved for Future Use- The following keywords do not yet have any functionality but are reserved by Rust for potential future use:
  abstract, become, box, do, final, macro, override, priv, try, typeof, unsized, virtual, yield
- Raw identifiers are the syntax that lets code use keywords where they wouldn’t normally be allowed. To use a raw identifier prefix the keyword with r#.

***************************************************************{Variables}**************************************************************************************
- Variables in Rust are defined through the 'let' keyword. In Rust, variables are immutable by default, meaning once the variable is assigned a value, the value
  won’t change. To make a variable mutable, the 'mut' keyword is added before the variable name. This allows the variable to be mutated. A variable can be used only
  if it has been initialized. Variable bindings are immutable by default, but this can be overridden using the mut modifier.
- Rust provides type safety via static typing. Variable bindings can be type annotated when declared. However, in most cases, the compiler will be able to infer the
  type of the variable from the context, heavily reducing the annotation burden. Values (like literals) can be bound to variables, using the let binding.
- Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}.
- It is possible to declare variable bindings first and initialize them later, but all variable bindings must be initialized before they are used: the compiler forbids
  use of uninitialized variable bindings, as it would lead to undefined behavior. It is not common to declare a variable binding and initialize it later in the function.
  It is more difficult for a reader to find the initialization when initialization is separated from declaration. It is common to declare and initialize a variable
  binding near where the variable will be used.
- When data is bound by the same name immutably, it also freezes. Frozen data can’t be modified until the immutable binding goes out of scope.
- Multiple let expressions can define multiple variables with the same name, known as variable shadowing. Variable shadowing allows transforming variables
  without having to name the variables differently. Variable shadowing is also possible for values of different types. Shadowing means that the first variable
  is shadowed by the second, which means that the second variable is what the compiler will see when the code uses the name of the variable. In effect, the
  second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
- In Rust, blocks of code are delimited by curly brackets. The {} define the scope. A scope is the range within the program for which the item is valid.
- Putting '_' infront of an unused variable can suppress the unused variable warning. Also a macro can tell the compiler to ignore the warning.
- Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants
  and variables. The keyword 'mut' is not allowed with constants. Constants aren’t just immutable by default—they’re always immutable. They can be declared using
  the 'const' keyword instead of 'let', and the type of the value must be annotated. Constants can be declared in any scope, including the global scope, which
  makes them useful for values that many parts of code need to know about. The last difference is that constants may be set only to a constant expression, not
  the result of a value that could only be computed at runtime. Constants are valid for the entire time a program runs, within the scope in which they were
  declared.
- Rust’s naming convention for constants is to use all uppercase with underscores between the words.

****************************************************************{Type Casting}**************************************************************************************
- Rust is a statically and strongly typed language. Operations of two different types can result in overflow or other errors.
- Type Casting allows to convert a type from one to other. Explicit type conversion is required when going from lower to higher types. An overflow may occur
  when going from higher to lower.
*/
use std::io;

#[allow(unused_variables)]
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    println!("Hello, world!");
    assert!(r#match("foo", "foobar"));

    // Rust-by-example
    {
        let an_integer = 1u32;
        let a_boolean = true;
        let unit = ();

        // copy `an_integer` into `copied_integer`
        let copied_integer = an_integer;

        println!("An integer: {:?}", copied_integer);
        println!("A boolean: {:?}", a_boolean);
        println!("Meet the unit value: {:?}", unit);

        // The compiler warns about unused variable bindings; these warnings can
        // be silenced by prefixing the variable name with an underscore
        let _unused_variable = 3u32;

        let _immutable_binding = 1;
        let mut mutable_binding = 1;

        println!("Before mutation: {}", mutable_binding);

        // Ok
        mutable_binding += 1;

        println!("After mutation: {}", mutable_binding);

        // Error! Cannot assign a new value to an immutable variable
        // _immutable_binding += 1;

        // This binding lives in the main function
        let long_lived_binding = 1;

        // This is a block, and has a smaller scope than the main function
        {
            // This binding only exists in this block
            let short_lived_binding = 2;

            println!("inner short: {}", short_lived_binding);
        }
        // End of the block

        // Error! `short_lived_binding` doesn't exist in this scope
        // println!("outer short: {}", short_lived_binding);
        // FIXME ^ Comment out this line

        let shadowed_binding = 1;

        {
            println!("before being shadowed: {}", shadowed_binding);

            // This binding *shadows* the outer one
            let shadowed_binding = "abc";

            println!("shadowed in inner block: {}", shadowed_binding);
        }
        println!("outside inner block: {}", shadowed_binding);

        // This binding *shadows* the previous binding
        let shadowed_binding = 2;
        println!("shadowed in outer block: {}", shadowed_binding);

        println!("outer long: {}", long_lived_binding);

        // Declare a variable binding
        let a_binding;

        {
            let x = 2;

            // Initialize the binding
            a_binding = x * x;
        }

        println!("a binding: {}", a_binding);

        let another_binding;

        // Error! Use of uninitialized binding
        // println!("another binding: {}", another_binding);
        // FIXME ^ Comment out this line

        another_binding = 1;

        println!("another binding: {}", another_binding);

        let mut _mutable_integer = 7i32;

        {
            // Shadowing by immutable `_mutable_integer`
            let _mutable_integer = _mutable_integer;

            // Error! `_mutable_integer` is frozen in this scope
            // _mutable_integer = 50;
            // FIXME ^ Comment out this line

            // `_mutable_integer` goes out of scope
        }

        // Ok! `_mutable_integer` is not frozen in this scope
        _mutable_integer = 3;
    }

    let apples = 5; // immutable
    let mut bananas = 5; //mutable
    bananas += 10;
    println!("There are {apples} apples and {bananas} bananas.");

    // Variable Shadowing
    let foo = 10;
    println!("The value of foo is {foo}");
    let foo = foo * 2;
    println!("The value of foo is {foo}");
    let _spaces = "   ";
    let _spaces = _spaces.len();
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    #[allow(dead_code)]
    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

}
