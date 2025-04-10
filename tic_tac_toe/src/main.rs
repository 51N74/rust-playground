use std::io;

fn main() {
    let mut board = [[' '; 3]; 3]; // สร้างบอร์ด 3x3 ด้วย array 2D
    let mut current_player = 'X'; // เริ่มด้วยผู้เล่น X
    let mut moves = 0; // นับจำนวนตาที่เล่น

    loop {
        print_board(&board); // แสดงบอร์ด
        let (row, col) = get_player_move(&board, current_player); // รับ input จากผู้เล่น

        // อัพเดทบอร์ด
        board[row][col] = current_player;
        moves += 1;

        // เช็คว่ามีผู้ชนะไหม
        if check_winner(&board, current_player) {
            print_board(&board);
            println!("ผู้เล่น {} ชนะ!", current_player);
            break;
        }

        // เช็คว่าเสมอไหม (เล่นครบ 9 ตาแล้ว)
        if moves == 9 {
            print_board(&board);
            println!("เกมเสมอ!");
            break;
        }

        // สลับผู้เล่น
        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

// ฟังก์ชันแสดงบอร์ด
fn print_board(board: &[[char; 3]; 3]) {
    println!("  0 1 2");
    for i in 0..3 {
        print!("{} ", i);
        for j in 0..3 {
            print!("{}", board[i][j]);
            if j < 2 { print!("|"); }
        }
        println!();
        if i < 2 { println!("  -+-+-"); }
    }
    println!();
}

// ฟังก์ชันรับ input จากผู้เล่น
fn get_player_move(board: &[[char; 3]; 3], player: char) -> (usize, usize) {
    loop {
        println!("ผู้เล่น {}: ป้อนแถวและคอลัมน์ (เช่น 0 1):", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("เกิดข้อผิดพลาดในการอ่าน");

        let coords: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coords.len() == 2 {
            let row = coords[0];
            let col = coords[1];
            if row < 3 && col < 3 && board[row][col] == ' ' {
                return (row, col);
            }
        }
        println!("ตำแหน่งไม่ถูกต้องหรือถูกใช้ไปแล้ว! ลองใหม่.");
    }
}

// ฟังก์ชันเช็คว่ามีผู้ชนะไหม
fn check_winner(board: &[[char; 3]; 3], player: char) -> bool {
    // เช็คแถว
    for row in board.iter() {
        if row.iter().all(|&c| c == player) {
            return true;
        }
    }

    // เช็คคอลัมน์
    for col in 0..3 {
        if (0..3).all(|row| board[row][col] == player) {
            return true;
        }
    }

    // เช็คแนวทแยง
    if (0..3).all(|i| board[i][i] == player) {
        return true;
    }
    if (0..3).all(|i| board[i][2 - i] == player) {
        return true;
    }

    false
}