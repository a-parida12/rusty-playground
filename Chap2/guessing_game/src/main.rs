// import files that are not in a prelude(standard imports)
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    
    // let -> creates a variable
    // rust-> immutable variables by default
    // import new from String -> creates a empty string
    
    
    // 
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        // read_line returns a io::Result -> error handling -> returns Ok or Err

        println!("You guessed: {}", guess); // format strings is facilitated by {}
        // convert the guess variable from string to u32
        // parse method converts string to number; expect is error handling for the parse
        // added match arms 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Enter a number only!!");
                continue;
            }
        };
        // _ catch all values 

        match guess.cmp(&secret_number) {
            // patern matching and arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}