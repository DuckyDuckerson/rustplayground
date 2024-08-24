use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_number = r_gen();

    guessing_game(secret_number);
    // -------------------------
    println!("Goodbye!");
}


fn r_gen() -> u32{
    // Saw this being talked about on reddit so I wrote it in rust   
    let z = 0;
    let x = 1;
    let a = 1;
    let b = 2;
    let c = 5;
    let d = 7;
    let e = 10;
    let f = 15;

    let mut i = rand::thread_rng()
        .gen_range(x..=a);
    let mut y = rand::thread_rng()
        .gen_range(b..=d);

    let mut r = rand::thread_rng()
        .gen_range(z..=e);
    match r {
        1..=5 => {
            r = rand::thread_rng()
                .gen_range(i..=y);
            return r;
        }
        6..=10 => {
            i = rand::thread_rng()
                .gen_range(c..=e);
            y = rand::thread_rng()
                .gen_range(z..=f);
            r = rand::thread_rng()
                .gen_range(i..=y);
            return r;
        }
        _ => {
            i = rand::thread_rng()
                .gen_range(c..=f);
            y = rand::thread_rng()
                .gen_range(e..=f);
            r = rand::thread_rng()
                .gen_range(i..=y);
            return r;
        }
    };
}


fn guessing_game(secret_number: u32){

    let mut tries = 0;

    while tries < 3{
        println!("Guess the number! Between 1 and 15.");
        println!("Please input your guess.");
        
        let mut guess = String::new();
        println!("You guessed: {}", guess);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                tries += 1;
                println!("Please enter a number!");
                println!("{}/3 tries left", tries);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                tries += 1;
                println!("{}/3 tries left", tries);

            }
            Ordering::Greater => {
                    println!("Too big!");
                    tries += 1;
                    println!("{}/3 tries left", tries);
            }
            Ordering::Equal => {
                    println!("You win!");
                    break;
            }
        }
    }
    println!("The secret number is: {}", secret_number);
}
