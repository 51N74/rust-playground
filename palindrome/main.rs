use std::io;

fn main(){
    let mut input:String = String::new();
    //อ่านนข้อมูลผ่าน io
    println!("Please enter a word:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let word = input.trim();

    //เรียก function สำหรับตรวจคำ palindrome
    if is_palindrome(word){
        println!("{} is a palindrome", word);
    } else {
        println!("{} is not a palindrome", word);
    }

}



fn is_palindrome(word :&str)->bool{
    //สร้างตัวแปรมาเก็บค่าที่ได้จากการแปลงตัวอักษร .chars() แปลงเป็นตัวอักษรทีละตัว .filter() เลือกเฉพาะตัวอักษรกับเลข สุดท้ายแปลงกับเป็น string
    let cleaned_word:String = word.chars().filter(|c| c.is_alphanumeric()).collect();

    //สร้างตัวแปรสำหรับเก็บคำที่กลับคำแล้ว ().rev
    let reversed_word:String = cleaned_word.chars().rev().collect();

    //Check ว่าสองคำเหมือนกันไหม
    cleaned_word.eq_ignore_ascii_case(&reversed_word)
}
