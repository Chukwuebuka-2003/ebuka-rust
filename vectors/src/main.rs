// How to create a new vector
let v: Vec<i32> = Vec::new(); //Vectors are implemented using generics and Vec<T> type can hold
//any type of data. Rust provides the vec! macro, which will create a vector that holds the values
//given to it.

let v = vec![1, 2, 3];

//Updating vectors
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

// If you want to make any variable mutuable, you can use the mut keyword

// Reading elements of vectors: This can be possible with indexing or the get method
let third: &i32 = &v[2];
println!("The third element is {}", third);

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// The index value of 2 was used to get the third element because vectors are indexed by number
// starting from 0. & and [] gives a reference to the element at the index specified. get method
// with the index passed as an argument, we get an Option<&T> that can be used with match

// The get method when passed on an index that is out of range will return None without panicking

// Iterating over the elements in vectors
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// iterating over mutable refrences to each element in a mutable vector
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

// to change the value that the mutable refrence refers to, use the * operator to get the value in
// i, before using the += operator

// Using enum to store multiple types in a vector
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
]; // Rust needs to know what types will be in the vector at compile time so it knows exactly how
// much memory on the heap will be needed to store each element

// Using an enum plus match expression means that Rust will ensure at compile time that every
// possible case is handled. Pop method removes and returns the last value of a vector

// Dropping a vector drops its elements: a vector is freed when it goes out of scope


