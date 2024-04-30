// Enums are also known as enumerations
//Structs give you a way of grouping together related fields and data.
//Enums give you a way of saying a value is one of a possible set of values.
//Structs are subsets of enums. Any struct can be represented as one-variant enum.
//Enums can have the same in-memory representation as structs.

//An example
enum IpAddrKind {
    V4,
    V6
}  //IpAddrKind is now a custom data type that we can use in our code

//Enum values: Creating instances of the two variants of IpAddrKind
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// define a function that takes any kind of IpAddrKind
fn route(ip_type: IpAddrKind) {}

//call this function with either variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);

//rather than writing an enum inside a struct, we can put data directly into each enum variant

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

// another advantage of using enum rather than a a struct is that each variant can have different
// data types and amounts of associated data.

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr2::V4(127, 0, 0, 1);
let loopback = IpAddr2::V6(String::from("::1"));

// another example of enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// This enum has four variants with different types: Quit has no data associated with it at all.
// Move has named fields, like a struct does.
// Write includes a single string. 
// ChangeColor includes three i32 values.

//rewriting the Message enum in struct format 
struct QuitMessage; // no data associated with QuitMessage
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// If I was to use the different structs, each of which has its own type, It wouldn't be asy to
// define a function to take any of these kind of messages compared to the Message enum which is a
// single type.

//A similarity between enum and struct: As we're able to define methods on structs using impl, it's
//possible to define methods on enums.
//An example
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

//The body of this method would use self to get the value that we called the method on. 
//A variable m was created that has the value Message::Write(String::from("hello")), and that is
//what self will be in the body of the call method when m.call() runs.

//The Option Enum and Its Advantages Over Null Values
// The Option type encodes the very common scenario in which a value could be something or it could
// be nothing. If you request the first item in a list that's not empty, you get a value. If you
// request the first item in an empty list, you get None.

//The problem with null values is that if you try to use a null value as a not-null value, you'll
//get an error of some kind, Because null/not-nul property is pervasive. Rust doesn't have nulls,
//but it does have an enum that can encode the concept of a value being present or absent.
//The enum is Option<T> 
enum Option<T> {
    None,
    Some(T),
} //The <T> syntax is a generic type parameter
// In the code above, <T> means that the Some variant of the Option enum can hold one piece of data
// of any type and that each concrete type that gets used in place of T makes the overall Option<T>
// type a different type.
//An example of using Option values to hold number types and string types:
let some_number = Some(5);
let some_string = Some("Ebuka");
let absent_number: Option<i32> = None;

// The type of some_number is Option<i32>
// The type of some_string is Option<String>
// When we have a Some value, we know that a value is present and the value is held within the Some
// When we have a None value, we know that it means the same thing as null 
// Some(value): This variant holds a value of type T. None: This variant represents the absence of a value.


// In order to use an Option<T> value, you want to have a code that will handle each variant.
// You want some code that will run only when you have a Some(T) value, and that code doesn't have
// a T value available. The match expression iss a control flow construct that does just this when
// used with enums: It will run different code depending on which variant of the enum it has, and
// that code can use the data inside the matching value.
