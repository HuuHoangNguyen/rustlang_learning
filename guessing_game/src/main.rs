use std::io;

fn main() 
{
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
        
    // println!("Result = {}", io::Result);    
    println!("You guessed: {}", guess);
    let x = 5;
    let y = 2*x;
    println!("x and y: {} and {}", x, y);

}
