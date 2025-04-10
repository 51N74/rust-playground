fn main(){
    let martrix:[[i32;3];3]= [[1,2,3],[4,5,6],[7,8,9]];
    for row in martrix.iter() {
        for val in row.iter() {
            print!("{} ", val);
        }
        println!();
    }

}