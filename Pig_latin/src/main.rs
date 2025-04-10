use std::io;

fn pig_latin(w:&str)->String{
    if w.is_empty() {
        return String::new();
    }

    let first_char:char = w.chars().next().unwrap();
    let vowels:[char;5]=['a','e','i','o','u'];

    if vowels.contains(&first_char){
        return format!("{}way",w)
    }else{
        let mut consonant:String = String::new();
        let mut remaining:String = String::new();
        let mut found_vowel:bool = false;

        for c in w.chars(){
            if !found_vowel && !vowels.contains(&c){
                consonant.push(c);
            }else{
                remaining.push(c);
                found_vowel = true;
            }
        }
        return format!("{}{}{}",remaining,consonant,"ay")
    }

}


fn main(){
    println!("Enter the word");

    let mut input:String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let word = input.trim();

    if word.is_empty(){
        println!("Please enter a valid word");
        return;
    }

    let pig_latin_word = pig_latin(word);
    println!("Pig Latin: {}", pig_latin_word);

}

