use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut turn = 1;
    let mut previously_spoken1: HashMap<u32, u32> = HashMap::new();
    let mut previously_spoken2: HashMap<u32, u32> = HashMap::new();
    let mut most_recent = 0;
    let line = reader.lines().next().unwrap().unwrap();
    for token in line.split(',') {
        let number = token.parse::<u32>().unwrap();
        previously_spoken1.insert(number, turn);
        most_recent = number;
        turn += 1;
    }

    while turn <= 30_000_000 {
        if turn == 2021 {
            println!("{}", most_recent);        
        }
        
        let number = if let Some(previous_turn) = previously_spoken2.get(&most_recent) {
            turn - 1 - previous_turn
        } else {
            0
        };
        
        if let Some(previous_turn) = previously_spoken1.get(&number) {
            previously_spoken2.insert(number, *previous_turn);
        }
        previously_spoken1.insert(number, turn);
        most_recent = number;
        turn += 1;
    }

    println!("{}", most_recent);
}
