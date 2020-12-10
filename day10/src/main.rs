use std::io::BufRead;
use std::collections::HashMap;

fn count_arrangements(adapters: & Vec<u32>) -> u64 {
    let mut memo: HashMap<u32, u64> = HashMap::new();
    memo.insert(adapters[0], 1);

    for index in 1..adapters.len() {
        let mut ways_to_connect = 0;
        for adapter in adapters.iter().filter(|adapter| *adapter > &adapters[index] && *adapter - adapters[index] <= 3) {
            ways_to_connect += memo.get(adapter).unwrap();
        }
        memo.insert(adapters[index], ways_to_connect);
    }

    *memo.get(&0).unwrap()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut adapters = Vec::new();
    adapters.push(0);
    let mut max_joltage = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let adapter = line.parse::<u32>().unwrap();

        if adapter > max_joltage {
            max_joltage = adapter;
        }

        adapters.push(adapter);
    }

    adapters.push(max_joltage + 3);

    adapters.sort();

    let mut one_jolt = 0;
    let mut two_jolt = 0;
    let mut three_jolt = 0;
    for (adapter_a, adapter_b) in adapters.iter().zip(adapters.iter().skip(1)) {
        match adapter_b - adapter_a {
            1 => {
                one_jolt += 1;                
            }
            2 => {
                two_jolt += 1;                
            }
            3 => {
                three_jolt += 1;                
            }
            _ => unreachable!(),
        }
    }

    println!("{}", one_jolt * three_jolt);

    adapters.reverse();
    println!("{}", count_arrangements(&adapters));
}
