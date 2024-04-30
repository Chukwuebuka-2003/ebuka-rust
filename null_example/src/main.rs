//In Rust, null values are not directly supported unlike Python or Java.
//Instead, Rust has a type called Option<T>, which can be one of two values: Some(T) or None.

//An example codebase
fn element_index(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index); // This will return the index if the target is found
        }
    }
    None // This will return None if the target is not found
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 6;

    match element_index(&numbers, target) {
        Some(index) => println!("Target found at index {}", index),
        None => println!("{} not found", target),
    }
}
