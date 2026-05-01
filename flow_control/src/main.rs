/// @Author: Mitul
/// @Description: Control Flow in Rust

/*
*****************************************************************{If/Else If/Else}******************************************************************************
- The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in
  most programming languages. The most common constructs that let the code control the flow of execution are if expressions and loops.
- An if expression allows to branch the code depending on conditions. All if expressions start with the keyword if, followed by a condition. Blocks of code
  associated with the conditions in if expressions are sometimes called arms. Optionally, an else expression can be include to give the program an alternative
  block of code to execute should the condition evaluate to false. If an else expression is mentioned and the condition is false, the program will just skip
  the if block and move on to the next bit of code.
- Rust will not automatically try to convert non-Boolean types to a Boolean.The code must be explicit and always provide it with a Boolean as its condition.
- Multiple conditions and arms can be used by combining if and else in an else if expression.
- Because if is an expression, it can be used on the right side of a let statement to assign the outcome to a variable provided the arms have similar
  expression types.
- Branching with if-else is similar to other languages. Unlike many of them, the boolean condition doesn’t need to be surrounded by parentheses, and each
  condition is followed by a block. if-else conditionals are expressions, and, all branches must return the same type.

*********************************************************************{Loops}*************************************************************************************
- It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop
  body to the end and then start immediately back at the beginning. Rust has three kinds of loops: loop, while, and for.
- The loop keyword tells Rust to execute a block of code over and over again forever or until told explicitly it to stop. Rust also provides a way to break
  out of a loop using code. The break keyword can be placed within the loop to tell the program when to stop executing the loop. The continue in a loop tells
  the program to skip over any remaining code in this iteration of the loop and go to the next iteration.
- Code can also use return from inside a loop. While break only exits the current loop, return always exits the current function.
- If there are nested loops, break and continue apply to the innermost loop at that point. A loop label can be optionally applied on a loop that can then be
  used with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single
  quote.
- It’s possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label
  must be passed to the break/continue statement.
- One of the uses of a loop is to retry an operation until it succeeds. If the operation returns a value though, you might need to pass it to the rest of the
  code: put it after the break, and it will be returned by the loop expression.
- A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the
  program calls break, stopping the loop. It’s possible to implement behavior like this using a combination of loop, if, else, and break. However, this
  pattern is so common that Rust has a built-in language construct for it, called a while loop. This construct eliminates a lot of nesting that would be
  necessary when used loop, if, else, and break. While a condition evaluates to true, the code runs; otherwise, it exits the loop.
- The while construct can be used to loop over the elements of a collection, such as an array. This approach is error prone though.
- As a more concise alternative, a for loop can be used to execute some code for each item in a collection.
- The for in construct can be used to iterate through an Iterator. One of the easiest ways to create an iterator is to use the range notation a..b.
  This yields values from a (inclusive) to b (exclusive) in steps of one.
- A semicolon at the end of the loop body makes it an expression which returns a value. While loop and for loop can not return a value.

*********************************************************************{for and iterators}*************************************************************************************
- The for in construct is able to interact with an Iterator in several ways. As discussed in the section on the Iterator trait, by default the for loop will
  apply the into_iter function to the collection. However, this is not the only means of converting collections into iterators. The into_iter, iter and iter_mut
  all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.
- iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
- into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available
  for reuse as it has been ‘moved’ within the loop.
- iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.

*/

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        n / 2
        // TODO ^ Try suppressing this expression with a semicolon.
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    //---------------------------------------------------------------------------------------

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    //---------------------------------------------------------------------------------------

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    //---------------------------------------------------------------------------------------

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    //---------------------------------------------------------------------------------------

    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    //---------------------------------------------------------------------------------------

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    //---------------------------------------------------------------------------------------

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    //---------------------------------------------------------------------------------------

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line

    //---------------------------------------------------------------------------------------

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn more_examples() {
    // If Expression
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loops
    // loop {
    //     println!("again!");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //  This will return a value of counter*2 (20)
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    let returned_value = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up 43;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!(
        "End count = {count} and Returned Value = {:#?}",
        returned_value
    );

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    let a = [100, 200, 300, 400, 500];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in a.iter() {
        println!("The Value is: {}", number);
    }
    println!("Excluding Upper Bound");
    for r in 1..4 {
        println!("{}!", r);
    }
    println!("Including Upper Bound using =");
    for r in 1..=7 {
        println!("{}!", r);
    }
}
