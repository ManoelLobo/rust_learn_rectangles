fn main() {
    let width = 10;
    let height = 15;

    let rect_area = area(width, height);

    println!("The area of the rectangle is {rect_area}")
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
