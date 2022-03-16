use std::io;
use rand::Rng;

fn main() {
    println!("Guessing game! 🤪");
    println!("Input your guess:");
    let mut a = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);
    
    io::stdin()
        .read_line(&mut a)
        .expect("failed to read line");
    
    println!("You guessed: {}", a);
}
