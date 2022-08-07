fn main() {
    let width = 10;
    let height = 15;

    let rect_area = area((width, height));

    println!("The area of the rectangle is {rect_area}")
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
