use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {secret_number}");

    loop {
        println!("Please input your guess");
    
        let mut guess_str = String::new();
    
        io::stdin()
          .read_line(&mut guess_str)
          .expect("Failed to read line");
    
        // initialize variable guess using match
        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,// if Ok it will return num
            Err(_) => continue//if Err with _ catchall will continue to next loop
        };
    
        println!("You guessed : {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
