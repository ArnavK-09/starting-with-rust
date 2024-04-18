// standard i/o
use std::io;

fn main() {
    // welcome msg
    println!(
        "
    █▀▀ █░█ █▀▀ █▀ █▀   █▀▀ ▄▀█ █▀▄▀█ █▀▀
    █▄█ █▄█ ██▄ ▄█ ▄█   █▄█ █▀█ █░▀░█ ██▄
    "
    );
    println!("Guess the number:- ");

    // variable to store user guess || new is an associated function of the String type.
    let mut guess_by_user = String::new(); // empty string

    // ask user for a number
    io::stdin()
        .read_line(&mut guess_by_user)
        .expect("No number found");
    println!("\nYou guessed number: {}", guess_by_user);
}
