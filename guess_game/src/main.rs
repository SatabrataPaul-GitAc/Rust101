use std::io;

fn main() {
    let x = 5;
    let y = 8;

    println!("Welcome to guess game.....");
    println!();
    println!("Guess the number!!!");

    let mut user = String::new();

    io::stdin()
    .read_line(&mut user)
    .expect("Failed to take input");

    println!("Your guess is : {}", user);
    println!();
    println!("The values of x and y are : {} and {} respectively", x, y);
}
