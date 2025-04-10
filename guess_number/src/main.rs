use rand::prelude::*;
use std::io;

fn main(){

let random_number = random_number();  

println!("Welcome to the Guessing Game!");

for i in 1..=100{
    let mut guess = String::new();
    println!("Please enter a number between 1 and 100.");
    guess.clear();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");
    println!("You guessed: {}", guess);

    if guess < random_number {
        println!("Too low! Try again.");
    } else if guess > random_number {
        println!("Too high! Try again.");
    } else {
        println!("Congratulations! You guessed the number!");
        break;
    }
}

}

fn random_number() -> u32 {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.random_range(1..=100);
    println!("The random number is: {}", random_number);
    random_number
}