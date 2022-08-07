struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    let rect_area = area((rectangle.width, rectangle.height));

    println!("The area of the rectangle is {rect_area}")
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
