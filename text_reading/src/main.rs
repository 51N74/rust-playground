use std::collections::HashMap;
use std::fs;

fn main(){
    let filename = "test_text.txt";
    let contents = fs ::read_to_string(filename).expect("failed to read file");

    let mut word_count:HashMap<String, usize> = HashMap::new();

    for word in contents.split_whitespace(){
        let cleaned_word = word.to_lowercase().trim_matches(|c:char| !c.is_alphanumeric()).to_string();

        if !cleaned_word.is_empty(){
            let count = word_count.entry(cleaned_word).or_insert(0);
            *count += 1;
        }
    }

    println!("คำที่ซ้ำในไฟล์: ");
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }

}