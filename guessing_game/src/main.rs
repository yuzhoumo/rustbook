fn main() {
    println!("Guess the number!");

    println!("Please input your guess.")

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .except("Failed to read line");

    println!("You guessed: {guess}")
}
