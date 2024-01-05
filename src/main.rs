use std::io;

fn main() {

    println!("Do you want to play a guessing game?");
    println!("Press 'y' if you want to play and 'n' if you don't");

    let response = evaluate_user_response();
    println!("{}", response);
}

fn play_game() { 
    let answer = 31;
    println!("Guess the number!");
    loop {
        let mut number_guessed = String::new();
        io::stdin().read_line(&mut number_guessed).expect("failed to read line");
        let conversion: i32 = number_guessed.trim().parse().expect("Gimme the right thing");
        if conversion > answer {
            println!("Go lower!");
        } else if conversion == answer {
            println!("That's the answer!");
            break;
        } else {
            println!("Go higher!");
        }
    }
    println!("Do you want to play again? If so press 'y', if not press 'n'");
}

fn evaluate_user_response() -> bool {
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response).expect("Failed to read line");
    let user_response = user_response.trim();

    user_response == "y"
}