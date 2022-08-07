struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    let rect_area = area(rectangle);

    println!("The area of the rectangle is {rect_area}");

    // Will not work because `area()` got ownership of `rectangle`
    // let width = rectangle.width;
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
