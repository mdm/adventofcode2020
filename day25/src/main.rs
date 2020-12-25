use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut public_keys = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        public_keys.push(line.parse::<u64>().unwrap());
    }

    let mut loop_sizes = Vec::new();

    for i in 0..2 {
        let mut value = 1;
        let mut exponent = 0;
        while public_keys[i] != value {
            value = (value * 7) % 20201227;
            exponent += 1;
        }

        loop_sizes.push(exponent);
    }

    let mut encryption_key = 1;
    for _ in 0..loop_sizes[1] {
        encryption_key = (encryption_key * public_keys[0]) % 20201227;
    }

    println!("{}", encryption_key);
}
