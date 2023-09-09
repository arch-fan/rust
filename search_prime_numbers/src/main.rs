fn main() {
    let mut candidate = 2;

    loop {
        if is_prime(candidate) {
            println!("{candidate}");
        }
        candidate += 1;
    }
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as i64 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
