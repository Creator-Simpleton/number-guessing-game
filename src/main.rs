use std::io;

fn main() {
    let x = rand::random::<i8>();
    println!("Magic Number Generation Successful");
    println!("Range: -128 to 127");
    loop {
        println!("Guess the number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: i8 = guess.trim().parse().expect("Enter a valid number!");
        if guess > x {
            println!("The guess is bigger than the magic number.");
        } else if guess < x {
            println!("The guess is smaller than the magic number.");
        } else if guess == x {
            println!("You Won!");
            break;
        } else {
            println!("Retry.");
        }
    }
}
