use std::io;

#[warn(unused_imports)]
fn main() {
    println!("Guess the number!");

    println!("Please imput your guess.");

    let mut guess = String()::new;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}",guess);
}
