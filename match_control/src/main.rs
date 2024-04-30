// The match control flow construct:
// match allows one to compare a value against a series of patterns and execute
// code based on the result of the match.
// Patterns can be made up of literal values, variables, wildcards, ranges, or
// destructuring assignments
// Think of match like this: Values go through each pattern in a match, and at the first pattern
// the value fits, the value falls into the associated code block to be used during execution
// An example:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//An explanation of the match in the value_in_cents function:
//List the match keyword followed by an expression, such as the value coin. The coin here is the
//Coin enum that was defined above. 

//The match arms: An arm has two parts; a pattern and some code. From the code above, the first
//arm here has a pattern that is the value Coin::Penny and then the => operator that seperates the
//pattern and the code to run. The code in this case is the value 1. Each arm is separated by a
//comma. When the match expression executes, it compares the resultant value against the pattern of
//each arm, in order.

// If a pattern matches the value, the associated code block is executed.
// If that pattern doesn't match, the match expression will check the next arm.

//The code associated with each arm is an expression, and the resultant value of the expression in
//the matching arm is the value that gets returned for the entire match expression. If you want to
//run multiple lines of code in a match arm, you must use curly brackets, and the comma following
//the arm is then optional

// Patterns That Bind to Values: This another useful feature of match arms, they can bind to the
// parts of the values that can match the pattern. This is how it's possible to extract values out
// of enum variants

//An example
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// write a function that takes an Option<i32> and if there's a value inside, adds 1 to that value.
// If there's not a value inside, the function should retirn None value and not attempt to perform
// any operations on it.

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// An explanation of the code above:
// Calling plus_one(five), the variable x in the body of plus_one will have the value Some(5). We
// then compare that against each match arm (None => None). The Some(5) value doesn't match the
// pattern None, so we continue to the next arm (Some(i) => Some(i + 1)). Some(5) match Some(i),
// the i binds to the value contained in Some, so i takes the value 5. The code in the match arm is
// then executed, so we add 1 to the value of i and create a new Some value with our total 6
// inside.
// Now look at where X is None, we enter a match and compare to the first arm (None => None), it
// matches and there's no value to add to, so the program stops and returns the None value on the
// right side of =>. Because the first arm matches, no other arms are compared.

// Combining match and enums is useful in many situations

// The arms patterns in matches must cover all possibilities. Matches in Rust are exhaustive. We
// must exhaust every last possibility in order for the code to be valid.

// Catch-all Patterns and the _ Placeholder: The catch-all pattern can be used to match any value
// and bind the value to a variable. This is useful when you want to bind the value to a variable

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num: u8) {}

// we have to put the catch-all pattern last because the patterns are evaluated in order
// If we put the catch-all pattern first, the code will fail with an error

// Rust also have a pattern called _ which matches any value and does not bind to that value.
// an example:
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {} 

// Using the unit value (The empty tuple)
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

// In the code above, we're indicating to Rust that we aren't going to use any value that doesn't
// match a pattern in an earlier arm, and we don't want to run any code in this case

