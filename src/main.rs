use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number between 1 and 20");

    let secret_number = rand::thread_rng().gen_range(1..20);

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error getting message");

        let int_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your input :{}", int_guess);

        match int_guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
