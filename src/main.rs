use std::io;

fn main() {

    println!("Do you want to play a guessing game?");
    println!("Type 'yes' if you want to play and 'no' if you don't");

    'game: loop {

        let mut wants_to_play = String::new();
        let mut will_play = false;
        io::stdin().read_line(&mut wants_to_play).expect("failed to read line");

        let wants_to_play = wants_to_play.trim();

        if wants_to_play == "yes" {
            will_play = true;
        } else {
            will_play = false;
        }

        if !will_play {
            break;
        }

        let answer = 31;
        println!("Guess the number!");

        loop {
            let mut number_guessed = String::new();
            io::stdin().read_line(&mut number_guessed).expect("failed to read line");
            let conversion: i32 = number_guessed.trim().parse().expect("Gimme the right thing");
            if conversion > answer {
                println!("Go lower!");
            } else if (conversion == answer) {
                println!("That's the answer!");
                break;
            } else {
                println!("Go higher!");
            }
        }
    }
}
