use std::io;
use rand::Rng;

fn main() {
    println!("type a number to generate a random number between 1 and x");
    println!("when you're done, type 'exit' to exit");

    loop {
        // this line sets the users input as a variable named 'input'
        let mut input = String::new();

        // this block code will read the line and the .expect() line is there to handle an error and give an error message
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        // this block will check if you typed "exit" and when you do exit the program
        match input.trim() {
            "exit" => std::process::exit(0),
            _ => {}
        }

        // this block will check if the users input is a number, if it is not a number then it will ignore the line
        let input: i128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // now this line generates a random number in between 1 and the users input
        let rnum = rand::thread_rng().gen_range(1..={input});

        // and of course we print the number
        println!("{rnum}");
    }
}

