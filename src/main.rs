use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
use std::ops::Range;

use rand::{Rng, thread_rng};

fn input(prompt: &str) -> String {
    let mut buff = String::new();
    print!("{prompt}");
    stdout().flush().expect("Error in flushing prompt!");
    stdin().read_line(&mut buff).expect("Error reading input!");
    buff.trim().to_string()
}

fn get_number(prompt: &str) -> Option<isize> {
    let r = input(prompt);
    let Some(i) = r.parse().ok() else {
        return None;
    };
    Some(i)
}

fn get_random_number(k: Range<u8>) -> u8 {
    let mut tr = thread_rng();
    tr.gen_range(k)
}

fn main() {
    let mut guess_limit = 10;

    println!("=============================================");
    println!("|\t\t\tGuess the number game\t\t\t|");
    println!("---------------------------------------------");
    println!("|\t\tYou have to guess the randomly\t\t|");
    println!("|\t\tgenerated number.\t\t\t\t\t|");
    println!("|\t\tYou have a guess limit of {guess_limit}\t\t|");
    println!("|\t\tguesses.\t\t\t\t\t\t\t|");
    println!("=============================================");

    let g = get_random_number(1..100);

    while guess_limit > 0 {
        let i = get_number("\nGuess: ");

        if let None = i {
            println!("\nPlease enter a number!");
            continue;
        }

        match (i.unwrap() as u8).cmp(&g) {
            Ordering::Less => {
                println!("\nLess than the random number.\nTry again!");
                guess_limit -= 1;
                println!("Remaining guesses: {guess_limit}");
                continue;
            }
            Ordering::Equal => {
                println!("\nCongratulations!\nYou've guessed the random number.\n");
                break;
            }
            Ordering::Greater => {
                println!("\nGreater than the random number.\nTry again!");
                guess_limit -= 1;
                println!("Remaining guesses: {guess_limit}");
                continue;
            }
        }
    }
}
