
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the Number!");
    let secret_number = rand::random_range(1..=10);

    loop{
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("That guess was too small!\n"),
            Ordering::Greater => println!("That guess was too big!\n"),
            Ordering::Equal => {
                println!("\nYou Win!");
                break;
            }
        }
    }

    
}
