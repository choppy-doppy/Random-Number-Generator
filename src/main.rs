use std::io;
use rand::Rng;

fn main() {
    println!("Pick a number");

    /*
    this variable is marked with 'mut' which means it is mutable and cannot be changed later in the code
    the variable made will store the value from your input
     */

    let number = String::new();

    let number: u128 = number.trim().parse().expect("Type a number");

    let rnum = rand::thread_rng().gen_range(1..={number});

    // this block reads the input from the end user, the .expect() line is there to handle an error properly
    io::stdin()
        .read_line(&mut rnum)
        .expect("Failed to read line");

    println!("{number}");

    println!("random number: {rnum}");
}

