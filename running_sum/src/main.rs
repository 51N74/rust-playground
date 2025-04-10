use std::io;

fn main(){
    let mut input:String = String::new();
    let mut sum:i32 = 0;
    println!("please enter a string of digits or 'stop' to end the program");    

    loop {
    
    io::stdin().read_line(&mut input).expect("Failed to read line"); 

        if input.trim()=="stop"{
            println!("The sum of the digits is: {}", sum);
            println!("Stopping the program");
            break;
        }
        else{
            
            for i in input.trim().split_whitespace(){
                if let Ok(num) = i.parse::<i32>() {
                    sum += num;
                }
            }
            
        }
        input.clear();
    }
 
}