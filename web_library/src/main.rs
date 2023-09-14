use web_library;
fn main() {

    let header = web_library::header();
    let body = web_library::body();

    println!("{header}");
    println!("{body}");
}