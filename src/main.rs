struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    // One less variable assignment :)
    println!("The area of the rectangle is {}", area(&rectangle));

    // Would work now because `area()` got only a reference for `rectangle`
    // let width = rectangle.width;
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
