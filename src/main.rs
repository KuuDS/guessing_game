use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Welcome Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input your answer: ");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Invalid input");

        let guessing: i32;
        match buf.trim().parse::<i32>() {
            Ok(num) => guessing = num,
            Err(_) => continue,
        };

        match guessing.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Less => println!("Too small!!!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        };
    }
}
