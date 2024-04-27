// Methods are similar to functions, they're declard with the 'fn' keyword and a name, they can
// have parameters and a return value, has code that can run when the method is called from
// somewhere else. But unlike functions, methods are defined within the context of a struct or
// an enum. Its first parameter is always self, which is a reference to the struct or enum
// itself.

//Defining methods
//struct Rectangle {
//  width: u32,
// height: u32,
//}

//impl Rectangle {
//  fn area(&self) -> u32 {
//    self.width * self.height
// }
//}

//fn main() {
//  let rect1 = Rectangle {
//    width: 40,
//  height: 150,
// };

// println!(
//   "The area of the rectangle is {} square pixels.",
//    rect1.area()
//  );
//}

// This is a brief explanation of the code above: To define the funtion within the context of
// Rectangle, we start with an impl (implementation) block, and then we define the method. The
// method syntax goes after an instance: We add a dot followed by the method name, parentheses
// and then the parameters/arguments.

//In the signature for area, we use &self instead of rectangle: &Rectangle. The &self is actually
//short for self: &Self. Within an impl block, the self keyword is used to refer to the
//instance of the struct that the method is being called on.

// We chose &self here for the same reason we used &Rectangle in the function version: we don't
// want to take ownership, and we just want to read the data in the struct, not write to it.
// Having a method that takes ownership of the instance by using just self as the first parameter
// is rare; this technique is usualy used when the method transforms self into something else and
// you want to prevent the caller from using the original instance after the transformation.

//The main reason for using methods instead of functions is that methods are much more
//convenient to use and understand. Not having to repeate the type of self in every method's
//signature is for organization. All the things we can do with an instance of a type has been put
//in one impl block rather than making future users of our code search for capabilities of
//Rectangle in various places in the library we provide.

//giving a method the same name as one of the struct's fields
//impl Rectangle {
//fn width(&self) -> bool {
//      self.width > 0}
//}

//fn main() {
//   let rect1 = Rectangle {
// width: 50,
//   height: 200,
// };

//   if rect1.width() {
//  println!("The rectangle has a nonzero width; it is {}", rect1.width);
//    println!("The rectangle has a nonzero height; it is {}", rect1.height);
//  }
//}

// Rust doesn't have an equivalent to the -> operator in C or C++, so we can't use it as a
// method's. Instead Rust has a feature called automatic refrencing and dereferencing.
// Here's how it works: when you call a method with object.something(), Rust automatically adds in
// &, &mut, or * so object matches the signature of the method.

// Methods with more parameters
// The rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block for Rectangle
impl Rectangle {
    // Method to check if one rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 70,
        height: 150,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 90,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 70,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Structs lets you create custom data types which are meaningful for your domain. By using
// structs, you can keep associated pieces of data connected to each other and name each piece
// of data so you can refer to them later. In impl blocks, you can define functions that are
// associated with your type, and methods are a kind of associated function that lets you specify
// the behavior that instances of your structs have
