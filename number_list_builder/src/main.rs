use std::io;

fn main() {
    let mut input = String::new();
    let mut number_list: Vec<i32> = Vec::new();
    println!("Please enter numbers");

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

         if input.trim() != "stop" && !input.trim().is_empty() {
            number_list.push(input.trim().parse::<i32>().unwrap());
            input.clear();
        } else if input.trim() == "stop" {
            println!("You entered: {:?}", number_list);
            break;
        }
    }
}
