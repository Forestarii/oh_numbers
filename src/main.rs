
//game_about_numbers

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main()
{
    println!("Guess the number!");

    let random_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number: {}", random_number);

    println!("Please, input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess:i32 = guess.trim().parse().expect("Type a number.");

    println!("Your guess: {}", guess);

    match guess.cmp(&random_number)
    {
        Ordering::Less => println!("Too small..."),
        Ordering::Equal => println!("You have won!"),
        Ordering::Greater => println!("Too big!"),
    }
}


