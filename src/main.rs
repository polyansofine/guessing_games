use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Hello guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The Secret number is: {}",secret_number);
    loop {
        
        println!("Please input guessing value");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Please input correct number"); continue;},
        } ;
        println!("you guess: {}",guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win");break;}
        }
    }

}
