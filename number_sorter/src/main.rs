use std::io;

fn main() {
  let mut input = String::new();
  let mut list:Vec<i32>=Vec::new();

  println!("Enter 5 numbers for the list:");

  for _i in 1..=5{
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    list.push(input.trim().parse().expect("Please enter a number"));
    input.clear();
  }
  list.sort();

  println!("List: {:?}", list);

}
