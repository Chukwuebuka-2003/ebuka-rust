// Control flow operations in Rust

// If expressions: It allows you to branch your codee depending on conditions

// fn main() {
//let number = 10;

//  if number < 25 {
//    println!("This condition is true");
// } else {
//    println!("This condition is false");
//  }
//}

// it's important to remember that the condition must be a boolean, if you don't use boolean, you
// will get an error

//fn main() {
//   let number = 30;

// if number {
//   println!("number is thirty")
//  }

//fn main() {
// let num = 90;

// if num % 14 == 0 {
//   println!("this is divisible by 14");
// } else if num % 7 == 0 {
//     println!("this is divisible by 7");
// } else if num % 17 == 0 {
//    println!("divisible by 17");
// } else {
//    println!("This number : {num} can't be divided by either 14, 7 or 17")
//  }
//}

// using too many else if expressions can clutter your code, it's better to refactor your code

// Using if in a let statement

// Using If in a let statement

//fn main() {
//  let condition = true;
// let number = if condition { 5 } else { 9 };

//  println!("The value of the number is: {number}");
//}

// when the if and else statements have value types that are incompatible, it leads to an error
// because variables must have a single type

// Reptition with Loops
// The loop keyword tells Rust to execute a block of code over and over again forever until you
// explicitly tell it to stop

//fn main() {
//    loop {
//       println!("Ebuka");
// }
// }

// Returning Values from Loops

//fn main() {
// let mut counter = 0;

//let result = loop {
// counter += 1;

//  if counter == 10 {
//      break counter * 30;
//    }
//  };

//println!("The result is {result}");
//}

// The code above is a loop that increments a counter and breaks when the counter reaches 10 and
// multiplies the counter by 30

// Controlling Loops with while
// While loops let you run a block of code repeatedly as long as a condition is true.

//fn main() {
// let mut number = 2;

//   while number != 0 {
//println!("{number}!");

//  number -= 1;
//}

//  println!("My name is Chukuebuka, i'm a Rust developer");
//}

// While a condition evaluates to true, the code runs otherwise it exits the loop

// Looping through an array

//fn main() {
//  let b = [100, 200, 300, 400, 500];
// let mut index = 0;

// while index < 5 {
//   println!("The value is : {}", b[index]);

// index += 1;
// }
//}

// This code above can be slow, because the compiler adds runtime checks to see if the index is
// within the bounds of the array

// A better option might be using a for loop

//fn main() {
//let a = [10, 20, 30, 40, 50];

// for i in a {
//    println!("The value is: {i}");
//  }
//}

// This for loop above is better than the while loop because it doesn't have to check if the
// index is within the bounds of the array

// for loops are commonly used in Rust due to its safety and conciseness
fn main() {
    for number in (1..7).rev() {
        println!("{number}!");
    }
    println!("I'm Chukuebuka and I love Rust");
}
