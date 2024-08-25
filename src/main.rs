use std::{io::{self, Write}, time::Duration};
use rand::Rng;
use std::cmp::Ordering;
use std::thread;


fn main() {
    let p_time = 50;

    typewriter_r("Guessing Game", p_time);
    typewriter_l("Hello! Welcome to the guessing game!", p_time);

    fralse();

    let secret_number = r_gen();

    guessing_game(secret_number);
    // -------------------------
    println!("Goodbye!");
}


// --------------------------------------------

fn typewriter_r(text: &str, time: u64) {

    let mut text = text.chars();

    if let Some(first) = text.next() {

        print!("{}", first);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(time));

        typewriter_r(text.as_str(), time);
    } else {
        println!();
    }
}


fn typewriter_l (text: &str, time: u64){

    for c in text.chars(){
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(time));
    }
    println!();
}


fn fralse() {
    let minus_two = Number {
        odd: false,
        value: -2,
    };
    
    let n = Number {
        odd: true,
        value: 17,
    };
    
    println!("{}, {}", minus_two.is_positive(), n.is_positive());
    println!("{}", minus_two.odd);
}


struct Number {
    odd: bool,
    value: i32,
}
impl Number {
    fn is_positive(&self) -> bool {
        self.value > 0
    }
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
        print!("You guessed: {}", guess);
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
