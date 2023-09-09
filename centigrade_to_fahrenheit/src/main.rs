use std::io;

fn main() {
    let mut transform_to = String::new();

    println!("Transform to farenheit or celsius? \n\n \t 1) Farenheit \n \t 2) Celsius");
    io::stdin()
        .read_line(&mut transform_to)
        .expect("Failed to read line");

    let transform_to: u32 = match transform_to.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    if transform_to == 1 {
        let mut c = String::new();
        println!("How much centigrades?: ");
        io::stdin().read_line(&mut c).expect("Failed to read line");

        let c: f32 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                return;
            }
        };

        let result = convert_to_farenheit(c);
        println!("Result: {result}");
    } else if transform_to == 2 {
        let mut f = String::new();
        println!("How much farenheit?: ");
        io::stdin().read_line(&mut f).expect("Failed to read line");

        let f: f32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                return;
            }
        };

        let result = convert_to_centigrade(f);
        println!("Result: {result}");
    } else {
        println!("Invalid input. Please enter 1 or 2.");
    }
}

fn convert_to_centigrade(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn convert_to_farenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}
