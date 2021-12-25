use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("I choose a number between 0 and 100");
    let rand = rand::thread_rng().gen_range(0..=100);
    println!("Guess which number I choose!");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number :(");
                continue;
            }
        };
        println!("You guess {}", input);
        match input.cmp(&rand) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("to big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}
