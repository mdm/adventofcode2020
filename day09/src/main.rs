use std::io::BufRead;

fn is_valid(numbers: &Vec<u64>, index: usize, preamble_size: usize) -> bool {
    for i in (index - preamble_size)..index {
        for j in (index - preamble_size)..index {
            if j <= i {
                continue;
            }

            if numbers[i] + numbers[j] == numbers[index] {
                return true;
            }
        }
    }

    false
}

fn find_encryption_weakness(numbers: &Vec<u64>, invalid_number: u64) -> u64 {
    for start in 0..numbers.len() {
        for end in (start + 1)..numbers.len() {
            let sum: u64 = numbers.iter().skip(start).take(end - start).sum();
            if sum == invalid_number {
                let min = numbers.iter().skip(start).take(end - start).min().unwrap();
                let max = numbers.iter().skip(start).take(end - start).max().unwrap();
                return min + max;
            }
        }
    }

    0
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let preamble_size = std::env::args().nth(2).unwrap().parse::<usize>().unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut numbers = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        numbers.push(line.parse::<u64>().unwrap());
    }

    let mut invalid_number = 0;
    for i in preamble_size..numbers.len() {
        if !is_valid(&numbers, i, preamble_size) {
            invalid_number = numbers[i];
            break;
        }
    }

    println!("{}", invalid_number);    
    println!("{}", find_encryption_weakness(&numbers, invalid_number));
}