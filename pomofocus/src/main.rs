use std::time::Duration;
use std::thread::sleep;
use std::io;

fn main() {
    
    let short_break = 5 * 60; // ตั้งเวลา
    let long_break = 15 * 60; // ตั้งเวลา 
    let mut cycles = 0; // จำนวนรอบเริ่มต้น
    
    loop {
        
            println!("Please type 'start' to begin the Pomodoro session or type 'exit' to exit the Pomodoro Program.");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim(); // ลบช่องว่าง
            if input != "start" && input != "exit" { // ถ้าไม่ใช่ start หรือ exit
                println!("Invalid input. Please type 'start' to begin the Pomodoro session.");
                continue; // ถ้าไม่ใช่ให้วนลูปใหม่
            }else if input == "exit"{
                println!("Exiting the program.");
                break; // ถ้าเป็น exit ให้หยุดโปรแกรม
            }

        start_pomodoro(cycles); // เริ่ม Pomodoro

        cycles += 1; // เพิ่มจำนวนรอบ
        if cycles % 4 == 0 && cycles != 0 {
            println!("🛑 Time for a long break! {} minutes", long_break);
            for i in (1..=long_break).rev() {
                println!("⏰ Remaining: {} seconds", i);
                sleep(Duration::from_secs(1)); // หน่วงเวลา 1 วินาที
            }
        } else {
            println!("🛑 Time for a short break! {} minutes", short_break);
            for i in (1..=short_break).rev() {
                println!("⏰ Remaining: {} seconds", i);
                sleep(Duration::from_secs(1)); // หน่วงเวลา 1 วินาที
            }
        }
        
        println!("🍅 Pomodoro session {} completed!", cycles); // แจ้งเตือนเมื่อเสร็จสิ้น Pomodoro

        if cycles >= 16 { // ถ้าผ่าน 8 รอบให้หยุด
            println!("🎉 Congratulations! You've completed 8 Pomodoro sessions!");
            break;
        }
    }

}

fn start_pomodoro(c:i32)->i32 {

    let countdown = 25 * 60; // ตั้งเวลา 25 min
    println!("🍅 Pomodoro session {} started!", c+1); // แจ้งเตือนเมื่อเริ่ม Pomodoro
    for i in (1..=countdown).rev() {
        println!("⏰ Remaining: {} seconds", i);
        sleep(Duration::from_secs(1)); // หน่วงเวลา 1 วินาที
    }
   
    return c

}
   
