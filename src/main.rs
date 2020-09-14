use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's take a random number!");
    let num = rand::thread_rng().gen_range(1, 101);

    println!("Got a hidden number!");

    let mut tries = 0;

    loop {
        let mut input: String = "".to_string();

        println!("Let's guess what this number is:");
        io::stdin()
            .read_line(&mut input)
            .expect("Hmm, something is wrong :(");

        let input: i32 = input
            .trim()
            .parse()
            .expect("Something is wrong with your input!");

        match input.cmp(&num) {
            Ordering::Less => println!("The number you input is too small!"),
            Ordering::Greater => println!("The number you input is too big"),
            Ordering::Equal => {
                println!("Congratz! You guessed the right number!");
                break;
            }
        }

        tries = tries + 1;
    }
}
