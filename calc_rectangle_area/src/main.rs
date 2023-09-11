use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut width = String::new();

    println!("Introduce width: ");

    io::stdin()
        .read_line(&mut width)
        .expect("Error reading Width");

    let width: u32 = width.trim().parse().expect("Width must be a number!!");

    let mut height = String::new();

    println!("Introduce height: ");

    io::stdin()
        .read_line(&mut height)
        .expect("Error reading Width");

    let height: u32 = height.trim().parse().expect("Height must be a number!!");

    let rectangle = Rectangle { width, height };

    let area = calc_area(&rectangle);

    println!("The area of the rectangle {rectangle:?} is {area}");
}

fn calc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
