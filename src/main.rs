use fastrand;
// use std::io;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the letter!");

    // let secret_number = rand::thread_rng().gen_range(1..101);

    
// Generates lowercase alphabets from a--z


    let random: char = fastrand::lowercase();
    println!("{}",random);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // let mut guess = char::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            // .expect("Not a number");

        


        // let guess: u32 = "42".parse().expect("Not a number!");  



        let guess: char = match guess.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


