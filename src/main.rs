#![feature(never_type, once_cell)]
mod p8;

fn input(day: i64) -> String {
    std::fs::read_to_string(format!("./data/d{}.txt", day)).unwrap()
}

fn main() {
    p8::main();
}
