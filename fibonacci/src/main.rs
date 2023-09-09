fn main() {
    let mut sucession: i128 = 1;
    let mut old_sucession: i128 = 1;

    loop {
        let new_sucession: i128 = old_sucession + sucession;
        old_sucession = sucession;
        sucession = new_sucession;

        println!("{}", sucession);
    }
}
