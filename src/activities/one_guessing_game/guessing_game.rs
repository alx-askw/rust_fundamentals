use rand::Rng;
use std::{cmp::Ordering, io};
// !use println not print due to how the stdout buffer works - print will not do it in the expected order

pub fn guessing_game() {
    println!("Guess Number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("input guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error in guessing game");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
