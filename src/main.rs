use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number in 10 guesses or less!");
    println!("First, please select your difficulty.");
    println!("[easy] [medium] [hard]");

    let mut difficulty = String::new();

    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");

    let upper_bound: u32 = match difficulty.trim() {
        "hard" => 101,
        "medium" => 51,
        "easy" => 21,
        _ => 51,
    };

    let secret_number = rand::thread_rng().gen_range(1, upper_bound);

    let mut attempt: i32 = 0;

    loop {

        attempt += 1;

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! :D");
                println!("Attempts: {}", attempt);
                break;
            }
        }
        if attempt > 5 {
            println!("You lose! :(");
            break;
        }
    }
    println!("Would you like to play again? y/n");

    let mut play_again = String::new();

    io::stdin()
        .read_line(&mut play_again)
        .expect("failed to read line");

    let play_again = play_again.trim();
    if play_again == "y" {
        main();
    }
}
