use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    print!("secret number is {secret_number}\n");

    loop {
        println!("Guess the number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => print!("too big!"),
            Ordering::Equal => {
                print!("you are right!");
                break;
            }
        }
    }
}
