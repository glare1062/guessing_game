use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {

    let secret_number =  rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);


    loop {
        let mut guess = String::new();

        println!("Guess the number!");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read number");

        // let guess: u32 = guess.trim().parse().expect("type in a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("GUessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("COrrect guess");
                break;
            } ,
        }
    }



}
