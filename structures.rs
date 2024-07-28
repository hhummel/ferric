// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Area of a rectangle
fn area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left: Point {x: x1, y: y1}, bottom_right: Point {x: x2, y: y2} } = rectangle; 
    println!("x1: {} y1: {} x2: {} y2: {}", x1, y1, x2, y2);
    (x1-x2) * (y1 - y2)
}

// Square function
fn square(point: Point, length: f32) -> Rectangle {
    let another_point: Point = Point {..point};
    let rectangle: Rectangle = Rectangle { top_left: point, bottom_right: Point {x: another_point.x + length, y: another_point.y + length} };
    rectangle
}


fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..another_point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    // Test if left_edge is bound
    println!("left_edge: {:?}", left_edge);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Area: {:?}", area(_rectangle));

    let a_square = square(point, 10.0);
    println!("Area of square: {:?}", area(a_square));
}
