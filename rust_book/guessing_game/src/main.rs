use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Hello!");

    let sec_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess number> ");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed, no read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");


        match guess.cmp(&sec_num){
           Ordering::Less => println!("Too low ^"),
           Ordering::Greater => println!("Too large ~"),
           Ordering::Equal => {
               println!("Correct! You win!");
               break;
            }
        }
    }
}
