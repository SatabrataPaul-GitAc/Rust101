use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);
    
    println!("Welcome to guess game.....");
    println!();
    println!("Guess the number!!!");

    loop{
        println!("Enter your guess: ");
        let mut user = String::new();
        io::stdin()
        .read_line(&mut user)
        .expect("Failed to take input");
        let user: i32 = user.trim().parse().expect("A number is expected!!!");

        println!("Your guess is : {}", user);
        println!();

        match user.cmp(&secret_number) {
            Ordering::Less=>println!("Guess is too small"),
            Ordering::Greater=>println!("Guess is too big"),
            Ordering::Equal=>{
                println!("Correct guess! Your WIN !!!");
                break;
            }
        }
    }
}
