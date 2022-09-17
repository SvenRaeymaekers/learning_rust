use std::io;

fn main() {
    println!("Hello, world");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read user input line.");

    println!("{}", user_input);
}
