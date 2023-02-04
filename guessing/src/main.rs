use std::io;

// main function
fn main() {

    // macro --println!
    println!("Guess the Number!");

    println!("Please input your guess.");

    // mutable
    // string type
    // new
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

        println!("You guess: {}", guess);
}
