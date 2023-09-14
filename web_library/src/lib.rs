pub mod workers;

pub fn header() -> String {
    String::from("My website")
}

pub fn body() -> String {
    workers::get_body::new()
}
