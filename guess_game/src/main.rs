use std::io;

fn main() {
    println!("Welcome to guess game.....");
    println!();
    println!("Guess the number!!!");

    let mut user = String::new();

    io::stdin()
    .read_line(&mut user)
    .expect("Failed to take input");

    println!("Your guess is : {}", user);
}
