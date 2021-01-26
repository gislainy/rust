use std::io;

fn main() {

    let mut a_str = String::new();
    let mut b_str = String::new();

    io::stdin()
        .read_line(&mut a_str)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut b_str)
        .expect("Failed to read line");

    let a: u32 = a_str
        .trim()
        .parse()
        .expect("Wanted a number");
    
    let b: u32 = b_str
        .trim()
        .parse()
        .expect("Wanted a number");

    let sum = a + b;
    println!("X = {}\n", sum);
}