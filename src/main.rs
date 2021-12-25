use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    let rand = rand::thread_rng().gen_range(1..101);
    println!("Guess!");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => println!("error: {}", error),
        }
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
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
