use std::io;

fn main(){
    println!("Please enter a number:");
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number=input.trim().parse::<i32>().unwrap();

    if number == 0{
        println!("Invalid number, Please enter positive number");
        
    }else if number%2 ==1{
        println!("The number {} is odd ",number)
    }else if number%2 ==0 {
        println!("The number {} is even ",number)
    }

}