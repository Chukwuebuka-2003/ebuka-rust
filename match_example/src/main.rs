fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Value of six: {:?}", six);
    println!("Value of none: {:?}", none);
}

// The code above is an implementation of match in Rust
