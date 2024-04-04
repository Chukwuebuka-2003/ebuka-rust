// you can make a variable mutable by writing mut in front of the variable
//fn main() {
    //let mut x = 10;
    
    //println!("The value of x is: {x}");
    
    //x = 6;

    //println!("The value of x is: {x}");
//}

// Constants: These are values that are bound to a name and are not allowed to change, you can't
// use mut with constants. They're declared using const keyword instead of the let keyword
// Rust naming convention for consts: USE ALL UPPERCASE WITH UNDERSCORES BETWEEN WORDS

// Shadowing: We can shadow a variable by using the same variable's name and repeating the use of
// the let keywords, we don't use mut here but we use let

//fn main() {
    //let x = 8;

    //let x = x + 1;

    //{
        //let x = x * 2;
        //println!("The value of the inner scope is: {x}");
    //}

    //println!("The value of x is: {x}");
//}

// Data types: Every value in Rust is of a certan data type, it tells Rust what kind of data is
// being specified so it knows how to work with that data.
// There exists two data type subsets: scalar and compound
// Scalar types: It represents a single value. Rust has four primary scalar types: Integers,
// floating-point numbers, booleans and characters. Rust has f32 and f64(32 and 64 bits). All
// floating-point types are signed, f32 is a single-precision float and f64 has double precision

//fn main(){
    //let x = 4.0; //f64

    //let y: f32 = 3.0; //f32

//}

// Numeric Operations

//fn main() {
    //addition
    //let sum = 5 + 10;

    // subtraction
    //let difference = 95.5 - 4.3;

    //multiplication
    //let product = 4*1000;

    //division
    //let quotient = 5789 / 454;

    // remainder
    //let remainder = 578 % 23;
//}

// The Boolean: bool, the main way to use bool is through conditionals(if expression)
//fn main() {
    //let t = true;

    //let f: bool = false; 
//}

// The Character type: char, made up of four bytes in size and represents a unicode scalar value
//fn main() {
    //let c='z';
    //let z: char = 'Z';
    //let heart_eyed_cat = 'cat';
//}


// Compound type: It can group multiple values into one type. Rust has(tuples and arrays)
// Tuple: a general way of grouping together a number of values with a variety of types into one
// compound type. It have a fixed length, once declared it can't be shrinked or increased

//fn main() {
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
//}

// the variable tup binds to the entire tuple because a tuple is considered a single compound
// element. To get the individual values out of a tuple, a pattern matching is used to destructure
// a tuple value.

//fn main() {
    //let tup = (500, 6.43, 29);

    //let (x,y,z) = tup;

    //println!("The value of y is: {y}");
    //println!("The value of x is: {x}");
    //println!("The value of y is: {y}");

//}

// Array type: Stores a collection ofmultiple values, every element must have the same type.
// Unlike arrays in other languages, arrays in Rust have a fixed length

//fn main() {
    //let b = [7,3,5,6,8,7];
    //let (c) = b;
    //println!("The value of c is: {c}");
//}

// Accessing array elements
//fn main() {
    //let a: [i64; 5] = [10,20,30,40,50];

    //let _first = a[0];
    //let _second = a[1];
    //println!("The first value: {_first}");
    //println!("The second value: {_second}");
//}

// Invalid Array Element Access
//use std::io;

//fn main() {
    //let a = [100, 200,300,400,500];

    //println!("Please enter an array index");

    //let mut index = String::new();

    //io::stdin()
        //.read_line(&mut index)
        //.expect("Failed to read line");

    //let index: usize = index
        //.trim()
        //.parse()
       // .expect("Index entered was not a number");

    //let element = a[index];

   // println!("The value of the element at index {index} is: {element}");
    
//}


