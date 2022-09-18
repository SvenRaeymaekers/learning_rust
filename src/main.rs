use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Can't read user input.");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2);
}
