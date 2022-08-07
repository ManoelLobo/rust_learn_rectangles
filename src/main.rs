fn main() {
    let rect_dimensions = (10, 15);

    let rect_area = area(rect_dimensions);

    println!("The area of the rectangle is {rect_area}")
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
