use std::fs;
use std::io;
fn main() {
    let filename = "example.txt";
    
	let content = fs::read_to_string(filename).expect("Failed to read fille");
    let mut word_list:Vec<String> = Vec::new();

    for word in content.split_whitespace() {
        let cleaned_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_string();

        if !cleaned_word.is_empty(){
            word_list.push(cleaned_word);
            println!("{:?}",word_list)
        }
    }

    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();

    let mut new_word_list:Vec<String> = Vec::new();

    if input == "u"{
        for w in &mut word_list{
            
            w.to_uppercase();
            new_word_list.push(w.to_uppercase());
            println!("{:?}",new_word_list);
            
        }    
    }else if input == "l"{
        for w in &mut word_list{
            
            w.to_lowercase();
            new_word_list.push(w.to_lowercase());
            println!("{:?}",new_word_list);
            
        }   
    }else{
        println!("Invalid input");
    }
}

