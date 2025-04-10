use std::time::Duration;
use std::thread::sleep;
use std::io;

fn main(){

    println!("Please enter your time:");
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");


    let countdown:i32 = input.trim().parse().expect("Please enter a valid number");
    if countdown < 0 {
        println!("Please enter a positive number.");
        return;
    }

    println!("Countdown from {} seconds", countdown);
    println!("Enter for starting");

    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim() != "" {
        println!("Invalid input. Please press Enter to start the countdown.");
        return;
    }
    println!("🚀 Countdown started!");

    for i in (1..=countdown).rev(){
        println!("⏳ {} seconds remaining", i);
        sleep(Duration::from_secs(1));
    }

    println!("🚀 Time's up!");
    sleep(Duration::from_secs(2));


}