use std::io;
fn main(){
println!("Welcome to the Rust Fibonacci Calculator ");

let mut input:String = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
let n: u32 = input.trim().parse().expect("Please enter a number");
println!("Fibonacci of {} is {}", n, fibonacci(n));
}


fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}