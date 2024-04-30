// The if let syntax lets you combine if and let into a less complex way to handle values that
// match one pattern while ignoring the rest. 
// An example:
let config_max = Some(3u8);
match config_max {
    Some (max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
// The code above matches on an Option<u8> value in the config_max variable but only wants to
// execute code if the value is the Some variant.

// The code above can be written in a shorter way using the if let syntax:
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}

// The syntax if let takes a pattern and an expression separated by an equal sign. It works the
// same way as a match, where the expression is given to the match and the pattern is its first arm.
// The code in the if let block isn't run if the value inside it doesn't match the pattern.

// Using if let means less typing, less indentation, and less boilerplate code. But you lose the exhaustive checking that match enforces.
// Choosing between using match and if let depends on what you're trying to accomplish in your particular situation
// and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

// Else can be used in if let. Here's an example that compares match and if let else  

let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}


// now using if let else syntax
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

// Conclusion: If you have a situation in which your program has logic that is too verbose to
// express using a match, know that if let can be used. When enum values have data inside them, you
// can use match or if let to extract and use those values, and this depends on how many cases you
// need to handle.

// Creating custom types to use in your API ensures type safety: the compiler will make certain
// your functions only get values of the type each function expects.

