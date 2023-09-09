fn main() {
    let mut count = 0;

    'main_loop: loop {
        count = count + 1;
        println!("{count}");

        if count >= 45 {
            break 'main_loop;
        }
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
