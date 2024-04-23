// Ownership Rules:
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time. When the owner goes out of scope, the value will be dropped.

//Variable scope
//fn main() {
//  let s = "hello"; //s is valid from this point forward
//}

// The string type : String literals are convenient, but not suitable for every situation that
// needs text, they're immutable
// Rust has a second string type that's called a String. This one manages data allocated on the
// heap and is able to store an amount of text that's unknown to you at compile time. You can
// create a string literal using the from function
// String::from The :: operator allows us to namespace this particular function under the String
// type rather than using it as a function directly. Strings can be mutated, but literals cannot.
// The difference is in how these two types deal with memory.

// Memory and Allocation: With the String type, in order to support a growing piece of text, we
// need to allocate an amount of space on the heap, unkwown at compile time, to store the text
// inside.

//The  Rust drop function, which is called when a variable goes out of scope, will free the
// memory that the variable was using.

// A string is made up of three parts: A pointer to the memory location that holds the text, a
// length, and a capacity. The lennth is how much memory is in use, and the capacity is how much
// memory is allocated, in bytes. The difference between length and capacity means that the
// capacity is the total amount of memory that the string is able to hold, while the length is the
// amount of actual text that's in use.

//Freeing memory twice can lead to memory corruption, which can potentially lead to secutity
//vulnerabilities

//fn main() {
// let s = String::from("hello");
// let s2 = s;
//  println!("{}, world", s);
//}

// Variables and Data Interacting with Clone
// Rust have a method called clone that will copy the value of the variable into another variable
// of the same type.
//fn main() {
// let s1 = String::from("hello");
// let s2 = s1.clone();
//  println!("s1 = {}, s2 = {}", s1, s2);
//}

// When you see a call to clone, you know that some arbitrary code is being executed, and that code
// may be expensive. It's an indicator that something different is going on in your program.

// Stack-Only Data: copy
//fn main() {
// let x = 5;
// let y = x;
//  println!("x = {}, y = {}", x, y);
//}

// We didn't need to call clone explicitly, because x and y were both i32 values.
// If a type implements the Copy trait, variables that use it don not move, but rather are
// trivially copied, which makes them still valide after assignemnt to another variable.
// Rust won't allow us to annotate a type with Copy if the type, or any of its parts, has
// implemented the Drop trait. If the type needs something special to happen when the value goes
// out of scope and we add the Copy annotation to that type, we'll get a compile-time error.

//These are the types that can implement Copy:
//primitive types: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, bool, char
//tuple types: (i32, i32)

//Returning multiple values with a tuple
fn main() {
    let s1 = String::from("Chukwuebuka is smoking Rust");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

//Rust has a feature for using a value without transferring ownership, called references.
