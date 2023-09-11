use std::io;

struct MinecraftCube {
    pixels: u8,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl MinecraftCube {
    fn new() -> Self {
        Self { pixels: 16 }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold_on_minecraft_cube(&self) -> bool {
        let minecraft_cube = MinecraftCube::new();
        self.width <= minecraft_cube.pixels as u32 && self.height <= minecraft_cube.pixels as u32
    }
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

    println!(
        "The area of the rectangle {rectangle:?} is {}",
        rectangle.area()
    );

    if rectangle.can_hold_on_minecraft_cube() {
        println!("The rectangle can hold inside a minecraft cube")
    } else {
        println!("The rectangle can't hold inside a minecraft cube")
    }
}
