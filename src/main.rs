
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number! (1-9)");

    let secret_number = rand::thread_rng().gen_range(1..10);

    let isDebug: bool = false;
    let mut numGuesses = 3;

    loop {
        
        if(isDebug) {
            println!("The secret number is: {}", secret_number);
        }
        
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // If we hit this point, the user did not guess the number correctly

        numGuesses -= 1;

        if (numGuesses > 0){
            println!("Number of guesses left: {}", numGuesses);
        } else {
            println!("You lost!");
            println!(":(");
            break;
        }
        


    }

}
