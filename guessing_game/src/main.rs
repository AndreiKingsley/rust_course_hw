fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn main() {
    println!("Hello! Guess a number from 1 to 100");

    let guessed_number: u8 = (random_int() % 100 + 1) as u8;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input_number: u8 = input.trim().parse().unwrap();

        if input_number == guessed_number {
            println!("Congratulations! You won!");
            break;
        } else if input_number < guessed_number {
            println!("The guessed number is greater the input")
        } else {
            println!("The guessed number is less the input")
        }
    }
}
