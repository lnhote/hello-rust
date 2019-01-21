use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,11);
    println!("The secret number is 1-10");
    println!("Please input your guess:");
    let mut result = -1;
    while result != 0 {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess_i : u32 = guess.trim().parse().expect("Please type a number!");
        match guess_i.cmp(&secret_number) {
            Ordering::Less => {println!("You guessed: {}, too small!", guess_i); result = -1;},
            Ordering::Greater => {println!("You guessed: {}, too big!", guess_i); result = 1;},
            Ordering::Equal => {println!("You guessed: {}, you win!", guess_i); result = 0;},
        }
    }
    
}
