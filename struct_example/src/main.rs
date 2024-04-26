//fn main() {
//   let width1 = 80;
//  let height1 = 120;

//    println!(
//  "The area of the rectangle is {} square pixels.",
//    area(width1, height1)
//  );
//}

//fn area(width: u32, height: u32) -> u32 {
//  width * height
//}

// Refractoring with Tuples
//fn main() {
//let rect1 = (80, 120);

//  println!("The area of the rectangle is {} square pixels", area(rect1));
//}

//fn area(dimensions: (u32, u32)) -> u32 {
//  dimensions.0 * dimensions.1
//}

// Tuples don't name their elements, so we have to index into parts of the tuple.

// Refractoring with Structs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 80,
        height: 120,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
