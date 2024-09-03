use std::{io::{self, Write}, time::Duration};
use rand::Rng;
use std::cmp::Ordering;
use std::thread;


const P_TIME: u64 = 50;


fn main() {
    // super large unsorted array to test sorting algorithms
    let unsorted = vec![1, 5, 3, 2, 4, 6, 8, 7, 9, 10, 15, 13, 12, 11, 14];

    insertion_sort(unsorted.clone());
    quick_sort(unsorted.clone());

    let secret_number = r_gen();

    typewriter_r("Guessing Game\n", P_TIME);
    typewriter_l("Hello! Welcome to the guessing game!", P_TIME);

    fralse();

    guessing_game(secret_number);
    // -------------------------
    println!("Goodbye!");
}


// --------------------------------------------

fn insertion_sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
    array
}


fn quick_sort(mut array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        return array;
    }

    let pivot = array.remove(0);
    let mut less = Vec::new();
    let mut greater = Vec::new();

    for x in array {
        if x <= pivot {
            less.push(x);
        } else {
            greater.push(x);
        }
    }

    let mut sorted = quick_sort(less);
    sorted.push(pivot);
    sorted.append(&mut quick_sort(greater));
    sorted
}



fn typewriter_r(text: &str, time: u64) {

    let mut text = text.chars();

    if let Some(first) = text.next() {

        print!("{}", first);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(time));

        typewriter_r(text.as_str(), time);
    } else {
        print!("");
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
        typewriter_r("Guess the number! Between 1 and 15.\n", P_TIME);
        typewriter_r("~> ", P_TIME);
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                tries += 1;
                println!("Please enter a number!, {}", guess);
                println!("{}/3 tries left", tries);
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                typewriter_r("Too small!", P_TIME);
                tries += 1;
                println!("{}/3 tries left", tries);

            }
            Ordering::Greater => {
                typewriter_r("Too big!", P_TIME);
                tries += 1;
                println!("{}/3 tries left", tries);
            }
            Ordering::Equal => {
                typewriter_r("You win!", P_TIME);
                break;
            }
        }
    }
    println!("The secret number is: {}", secret_number);
}
