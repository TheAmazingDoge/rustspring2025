fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        -1
    } else if guess > secret {
        1
    } else {
        0
    }
}

fn main() {
    let secret_number = 7;
    let mut guess = 3;

    loop {
        let result = check_guess(guess, secret_number);
        
        if result == 0 {
            println!("You win!");
            break;
        } else if result < 0 {
            println!("Too small!");
            guess += 1;
        } else {
            println!("Too big!");
            guess -= 1;
        }
    }
}
