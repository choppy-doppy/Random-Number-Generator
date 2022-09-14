use std::io;
use rand::Rng;

fn main() {
    println!("type a number to generate a random number between 1 and x");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input: i128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let rnum = rand::thread_rng().gen_range(1..={input});

        println!("{rnum}");
    }
}

