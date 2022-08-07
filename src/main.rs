#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    println!("The area of the rectangle is {}", rectangle.area());

    println!("The rectangle is {:?}", rectangle);

    let rectangle2 = Rectangle {
        width: 8,
        height: 6,
    };

    if rectangle.can_hold(&rectangle2) {
        println!("rectangle can hold rectangle2");
    }
}
