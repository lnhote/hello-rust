use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,11);
    println!("The secret number is 1-10");
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        // If this instance of io::Result is an Err value, 
        // expect will cause the program to crash,
        // and display the message that you passed as an argument to expect
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess_i : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please type a number:");
                continue;
            },
        };
        match guess_i.cmp(&secret_number) {
            Ordering::Less => println!("You guessed: {}, too small!", guess_i),
            Ordering::Greater => println!("You guessed: {}, too big!", guess_i),
            Ordering::Equal => {
                println!("You guessed: {}, you win!", guess_i); 
                break;
            }
        }
    }
    
}
