use std::env;

mod day1;

fn main() {
    if let Some(value) = env::args().nth(1) {
        match value.as_ref() {
            "1" => day1::solve(),
            _ => println!("Not a valid day: {}", value),
        }
    } else {
        println!("Specify day as first and only argument");
    }
}
