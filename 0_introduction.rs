/// @Author: Mitul
/// @Description: Introduction
/*
- Rust files always end with the .rs extension.
- To generate an object from the rust source file, the command 'rustc source_file' is executed. This creates an object file which can be run. After compiling
  successfully, Rust outputs a binary executable.
- The main function is special: it is always the first code that runs in every executable Rust program.
- The function body is wrapped in {}. Rust requires curly brackets around all function bodies.
- rustfmt is the tool present in the package that can be used to format the code.
- Rust style is to indent with four spaces, not a tab.
- Using a ! means that code is calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.
-  Rust is an ahead-of-time compiled language, meaning user can compile a program and give the executable to someone else, and they can run it even without
   having Rust installed.
- In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line. For comments that extend beyond
  a single line, include // on each line. Comments can also be placed at the end of lines containing code.
-
*/
fn main() {
    println!("Hello, World!");
}
