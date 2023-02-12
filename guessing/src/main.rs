use std::io;
use rand::Rng;

// main function
fn main() {

    // macro --println!
    println!("Guess the Number!");

    println!("Please input your guess.");

    // mutable
    // string type
    // new
    let mut guess = String::new();

let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is : {}", secret_number);

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

        println!("You guess: {}", guess);

        let x = 5;
        let y = 10;

        println!("X = {} and Y = {}", x,y);
}
