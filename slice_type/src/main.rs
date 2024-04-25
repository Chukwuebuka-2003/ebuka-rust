// Slices lets you create a contiguous sequence of elements in a collection rather than the whole
// collection. It's a kind of refrence, so it does not have ownership

//writing a function for a problem without using Slices

//fn main() {
//let s = String::from("hello world");

//  let _word = first_word(&s); // Prefixed with underscore to indicate intentional unused variable.
//}

//fn first_word(s: &String) -> usize {
// let bytes = s.as_bytes();
//  for (i, &item) in bytes.iter().enumerate() {
//    if item == b' ' {
//      return i;
//    }
//  }
//  s.len()
//}

// iter is a method that returns each element in a collection and enumerate wraps the result of
// iter and returns each element as a tuple of (index, value)

//String slices: A refrence to a part of a String
//The slice data structure stores the start and end of a slice, which corresponds to ending_index
//and starting_index

fn main() {
    let s = String::from("hello world");

    let word_slice = first_word(&s);

    match word_slice {
        Some(word) => println!("The first word is: {}", word),
        None => println!("No space character found in the string."),
    }
}

fn first_word(s: &str) -> Option<&str> {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return Some(&s[..i]);
        }
    }
    Some(s)
}

//if we have an immutable refrence to something, we cannot take a mutable refrence to it
