// Refrences and Borrowing: A refrence is like a pointer in  to that's isan address that we can follow to access the data stored at that address;
// that data is owned by some other variable; Unlike a pointer, a refrence is guaranteed to point to the data at a specific memory address
// a refrence is immutable by default, but we can make it mutable by using the &mut keyword

// fn main() {
//  let s1 = String::from("Ebuka");

// let len = calculate_length(&s1);

//   print!("The length of '{}' is {}", s1, len);
//}

//fn calculate_length(s: &String) -> usize {
//  s.len()
//} //Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens

// The & refrence syntax is an ampersand (&) that precedes a variable name, type, or
// expression,they allow you to refer or to store a value without taking ownership of it.
// It's opposite is the * dereference syntax, which is used to get the value from a
// reference.

//The explanation of the code above is that we are using the & operator to create a
// reference to the String s1. The reference is then passed to the calculate_length.
// The action of creating a reference is called borrowing. Refrences are immutable by
// default.

// Mutable Refrences: To make a refrence mutable we use the &mut keyword
//fn main() {
// let mut s = String::from("Hello");
//  change(&mut s);
//}

//fn change(some_string: &mut String) {
//  some_string.push_str(", World");
//}

// Mutable Refrences have one big restriction: If you have a mutable refrence to a
// value, you can have no other refrences to that value.
// The restriction prevents data race. A data race is a problem that occurs when
// two or more threads access the same data at the same time.
// A refrence scope starts from where it's introduced and continues through
// the last time it's used.

//fn main() {
//  let mut s = String::from("hello");
//  let r1 = &s;
//  let r2 = &s;
//  println!("{}, {}", r1, r2);

// let r3 = &mut s;
//  println!("{}", r3);
//}

// borrowing errors might seem to be frustrating, remember that it's the Rust compiler pointing out
// a potential bug early (at compule time rather than at runtime) and showing you exactly where the
// problem is.

// Dangling refrences: A dangling refrence is a refrence that refers to a value that
// has been deallocated.

//fn dangle() -> &String {
//  let s = String::from("hello");
// &s // When we return a reference to the String, s is dropped, its memory goes away
//}

fn main() {
    let s = no_dangle();
    println!("{}", s);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// Note: THis entire main.rs file is an implementation of Refrences and Borrowing.
// Refrences must always be valid, you can have either one mutable refrenece or any number of
// immutable refrences.
