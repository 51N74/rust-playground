use std::io;
fn main(){
    let mut word_list:Vec<String>=Vec::new();
    let mut count = 1;
    loop{
        let mut input = String::new();
        

        println!("Please enter a word (or type 'exit' to quit):");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();

        if input == "exit" {
            word_list.retain(|word| word != "exit");
            println!("You entered: {:?}", word_list);
            println!("You entered was count: {:?}", count);
            break;
        }

        if word_list.contains(&input){
            println!("You have already entered this word.");
            count += 1;
        } 

        word_list.push(input)

    }

}