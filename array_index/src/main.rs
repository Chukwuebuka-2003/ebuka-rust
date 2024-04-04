use std::io;

fn main() {
    let a = [10, 58, 39, 64, 93];

    println!("Please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);

}

// When attempting to access an element using indexng, the index must be within the bounds of the
// array. If the index is out of bounds, the program will panic. This is an example of Rust's
// memory safety in action.
