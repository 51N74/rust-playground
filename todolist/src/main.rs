use std::io;

fn main(){
let mut list:Vec<String>= Vec::new();
loop {
    println!("\n Todo Rist");
    println!("1. Add Task");
    println!("2. Check Task");
    println!("3. Remove Task");
    println!("4. Exit");

    let mut input:String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    let choice=input.trim();

    match choice {
        "1"=>{
                println!("Input your task");
                let mut task:String =String::new();
                io::stdin().read_line(&mut task).expect("Failed to readline");
                let task = task.trim().to_string();
                list.push(task.clone())
        }
        "2"=>{
            if list.is_empty(){
                println!("Not Have Task");
            }
            for(index,task)in list.iter().enumerate(){
                println!("{}: {}", index+1, task);
            } 
        }
        "3" =>{
            if list.is_empty(){
                println!("Not Have Task");
                continue;
            }

            println!("Enter Task to Remove");    
            let mut rm:String =String::new();
            io::stdin().read_line(&mut rm).expect("Failed to readline");
            let rm = rm.trim().to_string();

            match rm.trim().parse::<usize>(){
                Ok(index) if index >0 && index <= list.len()=>{
                    let remove_task = list.remove(index-1);
                    println!("Task {}: '{}' remove complete",index,remove_task)
                }
                _ =>{
                    println!("Invalid number please try again")
                }
            }

        }
        "4" =>{
            println!("See you next time");
            break;
        }
        _ => println!("Invalid choice. Please enter a number from 1 to 4."),

        }
    }

}
