use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    guessing_game();
    // -------------------------
    println!("Goodbye!");
}


fn guessing_game(){
    let secret_number = rand::thread_rng().gen_range(1..=10);

    let mut tries = 0;

    while tries < 3{
        println!("Guess the number! Between 1 and 10.");
        println!("Please input your guess.");
        
        let mut guess = String::new();
        println!("You guessed: {}", guess);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                tries += 1;
            }
            Ordering::Greater => {
                    println!("Too big!");
                    tries += 1;
            }
            Ordering::Equal => {
                    println!("You win!");
                    break;
            }
        }
    }
    println!("The secret number is: {}", secret_number);
}
