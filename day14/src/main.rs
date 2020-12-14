use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut part1_memory = HashMap::new();
    let mut part1_bitmask_set: u64 = 0;
    let mut part1_bitmask_clear: u64 = 0;
    let mut part2_memory = HashMap::new();
    let mut part2_bitmask_floating: u64 = 0;
    let mut part2_popcount_floating: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let re = regex::Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        if re.is_match(&line) {
            // execute command
            let caps = re.captures(&line).unwrap();
            let destination: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let value: u64 = caps.get(2).unwrap().as_str().parse().unwrap();

            part1_memory.insert(destination, (value | part1_bitmask_set) & !part1_bitmask_clear & 0xf_ffff_ffff);

            for i in 0..(2 as u64).pow(part2_popcount_floating) {
                let mut real_destination = 0;
                let mut popcount_position = 0;
                for bit in 0..36 {
                    if (part2_bitmask_floating & (1 << bit)) > 0 {
                        real_destination = real_destination | (((i & (1 << popcount_position)) >> popcount_position) << bit);
                        popcount_position += 1;
                    } else {
                        real_destination = real_destination | ((destination | part1_bitmask_set) & (1 << bit));
                    }
                }

                part2_memory.insert(real_destination, value);
            }
        } else {
            // update mask
            part1_bitmask_set = 0;
            part1_bitmask_clear = 0;
            part2_bitmask_floating = 0;
            part2_popcount_floating = 0;        
            let new_mask = line.split(" = ").nth(1).unwrap();
            for bit in new_mask.chars().rev().enumerate() {
                match bit.1 {
                    '0' => {
                        part1_bitmask_clear = part1_bitmask_clear | (1 << bit.0) & 0xf_ffff_ffff;
                    }
                    '1' => {
                        part1_bitmask_set = part1_bitmask_set | (1 << bit.0) & 0xf_ffff_ffff;
                    }
                    'X' => {
                        part2_bitmask_floating = part2_bitmask_floating | (1 << bit.0) & 0xf_ffff_ffff;
                        part2_popcount_floating += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    println!("{}", part1_memory.values().sum::<u64>());
    println!("{}", part2_memory.values().sum::<u64>());
}
