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

fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
