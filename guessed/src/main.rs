use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng()
        .gen_range(1, 101);

    println!("Krusty guess!");
    loop {
        println!("Your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect(dbg!("Failed to read"));

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Your guess => {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Guessed it!");
                break;
            },
        }
    }
}