// Rust uses snakecase as the conventional style for function and variable names, all letters are
// lowercase and underscores separate words

// Functions are defined by entering 'fn' followed by a set of parentheses. Rust doesn't care where
// you define your functions, only that they're defined somewhere in a scope that can be seen by
// the caller.

// fn main() {
// println!("Hello World!");

//  another_function();
//}

//fn another_function() {
//  println!("Another function");
//}

// Parameters: We can define functions to have parameters which are special variables that are part
// of a function signature. When a function has parameters, you can provide it with concrete values
// for those parameters. The concrete values could be referred as parameter or arguments

//fn main() {
//   another_function(5);
//}

//fn another_function(x: i32) {
//  println!("The value of x is: {x}");
//}

// The another_function has one parameter x  which is specified as i32. When 5 was passed into
// another_function, the compiler inferred that x was an i32, the println! puts 5 where the pair of
// curly brackets containing x was in the format string

// Defining multiple parameters in Rust, you need to separate the parameter declarations with
// commas

//fn main() {
//  print_labeled_measurements(800, 'h');
//}

//fn print_labeled_measurements(value: i32, unit_label: char) {
//  println!("The measurement is: {value}{unit_label}");
//}

// Because the function was called with 800 as the value and 'h' as the unit label, the
// print_labeled_measurements function will print 800h

// Statements and Functions:
// Functions bodies are made up of a series of statements optionally ending in an expression.
// Rust is an expression based language and this is an important distinction.
// Statements are instructions that perform some action and don't return a value.
// Expressions evaluate to a resultant value.
// Creating a variable and assigning a value to it using the let keyword is a statement. You can't
// assign a let statement to another variable once you have defined it. Doing this will lead to an
// error.

// Expressions can be part of statements. Calling a function is an expression, calling a macro is
// an expression. A new scope block created with curly brackets is an expression

//fn main() {
// let y = {
// let x = 5;
//  x + 1
//};

//  println!("The value of y is: {y}");
//}

// Expressions are not statements, they don't end with a semicolon.

// Functions with Return values:
// Functions can return values to the code that calls them. We don't name return values, but we
// declare them with the -> operator and specify the type of the value they return.

//fn five() -> i32 {
// 5
//}

//fn main() {
//  let x = five();

//println!("The value of x is: {x}");
//}

// There are no function calls, macros or even let statements in the five function. This is a valid
// function in Rust.
// let x = five(); shows that the five function is used to initialize a variable.
// The five function has no parameters and defines the types of the return value, but the body of
// the function is a lonely 5 with no semicolon because it's an expression whose value we want to
// return.

fn main() {
    let x = plus_three(90);

    println!("The value of x is: {x}");
}

fn plus_three(x: i32) -> i32 {
    x + 3
}
