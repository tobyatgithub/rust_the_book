use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello now!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is : {secret_number}");

    loop {
        println!("Type in a number to guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!"); 
                break;},
        }
    }
}
