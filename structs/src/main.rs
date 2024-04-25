// A struct/structure is a data type that contains a collection of fields
// A struct/structure can be declared using the keyword struct
// Structs are similar to tuples. Unlike with tuples, in a struct you'll name each piece of data so
// that's clear what the data is about
// This is the struct syntax
//struct User {
//    username: String,
//  email: String,
//    sign_in_count: u64,
//  active: bool,
//}

// to use a struct after it has been defined, we create an instance of that struct by specifying
// the struct name and the values for each field. We create an instance by using the struct name
// followed by curly brackets containing key: value pairs, where the keys are the names of the
// fields and the values are the data that we want to assign to that field
// another example

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("VtP8s@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("VtP8s@example.com");

    // creating instances from other instances (user1)
    //  let user2 = User {
    //     active: true,
    //    username: String::from("someusername123"),
    //    email: String::from("VtP8s@example.com"),
    //  sign_in_count: 1,
    //};

    // now using struct update syntax
    let user2 = User {
        email: String::from("VtP8s@example.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// to access a value from a struct we use the dot notation

// Tuple Structs: This have the added meaning the struct name provides but don't have names
// associated with their fields; rather, they just have the types of the fields.
// Tuple structs are useful when you want to give the whole tuple a name and make that tuple
// a different type than the individual elements of the tuple
// To define a tuple struct
//struct Color(i32, i32, i32);
//struct Point(i32, i32, i32);

//fn main() {
//  let black = Color(0, 0, 0);
//   let origin = Point(0, 0, 0);
//}
// a function that takes a parameter of type Color cannot take a Point as an argument, even though
// both types are made up of three i32 values.

// Unit-like structs: These are structs that don't have any fields at all
// They are useful for making empty structs, and when you need to implement a trait on some type
// but don't have any data that you want to store in the type itself.
// an example:
//struct AlwaysEqual;
//fn main() {
//  let subject = AlwaysEqual;
//}
