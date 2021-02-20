// dependencies
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// main is a special function that is ran first
fn main() {
    println!("Guess the number!");

    // declare secret_number variable (immutable by default)
    // gen_range lowerbound is inclusive and upperbound is exclusive
    // generates a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // {} is a placeholder that is interpolated into
    // println!("Multiple example: {}, {}, {}", x, y, z)
    // println!("The secret number is {}", secret_number);

    // infinite loop
    loop {
        println!("Please input your guess:");

        // declare guess a mutable String
        let mut guess = String::new();

        // ask for user input
        // expect is output when an error occurs
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // set guess to an unsigned 32 bit integer
        // remove whitespace with trim and attempt to convert string to u32 integer
        // error handling: ask for another guess if a non-number is entered
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        // compare guess to the secret number
        // break out of loop if they are equal
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
