#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    println!("The area of the rectangle is {}", area(&rectangle));

    println!("The rectangle is {:?}", rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
