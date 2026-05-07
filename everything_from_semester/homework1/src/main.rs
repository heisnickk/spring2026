fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }   
}

fn main(){
    let secret: i32 = 67;
    let mut guesses: i32 = 0;
    let mut guess: i32 = 5;

    loop {

        guesses += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {}: {} is correct!", guesses, guess);
            break;
        } else if result == 1 {
            println!("Guess {}: {} is too high.", guesses, guess);
            guess += 1;
        } else {
        println!("Guess {}: {} is too low.", guesses, guess);
            guess += 1;
        }

    
    }
    println!("It took {} guesses to find the secret number.", guesses);
}