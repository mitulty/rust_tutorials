/// @Author: Mitul
/// @Description: Expressions in Rust

/*
- A Rust program is (mostly) made up of a series of statements:
    fn main() {
    // statement
    // statement
    // statement
    }
- There are a few kinds of statements in Rust. The most common two are declaring a variable binding, and using a ; with an expression:
    let x = 5; // statement
    let y = 6; // statement
    let z = x + y; // statement
    fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;
    }
- Blocks are expressions too, so they can be used as values in assignments. The last expression in the block will be assigned to the place expression such as a local variable.
  However, if the last expression of the block ends with a semicolon, the return value will be ().
- The last statement in the main function is an expression without a semicolon, which is an expression that returns a value. In this case, the value of the expression is 11, 
  and it will be returned from the main function and printed to the console.
    fn main() {
    let x = 5;
    let y = 6;
    x + y // expression that returns 11
    }  
- Statements do not return a value. Expressions evaluate to a resulting value. The main way to create an expression is to use operators, such as `+` or `*`.
*/

#[allow(unused_must_use)]
#[allow(unused_variables)]
fn main() {
    let x = 5u32;
    
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        
        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
    
    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 as u32 * x;
    };
    
    let z =
    {
        // This expression returns `10`, and the semicolon is not needed here.
        2 as u32 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    // This will give an error 
    // x + y; // expression that returns 11
    println!("z is {:?}", z);

    // This will give an error as x is returned from the block and not assigned to a variable
    // It expects () but found u32
    // {
    //     x
    // }

    // This will work as the last expression is a statement and returns ()
    {}
}
    