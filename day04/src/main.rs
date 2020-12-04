use std::io::BufRead;
use std::collections::HashMap;

fn field_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            let value = value.parse::<u32>().unwrap();
            value >= 1920 && value <= 2002
        },
        "iyr" => {
            let value = value.parse::<u32>().unwrap();
            value >= 2010 && value <= 2020
        },
        "eyr" => {
            let value = value.parse::<u32>().unwrap();
            value >= 2020 && value <= 2030
        },
        "hgt" => {
            let re = regex::Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
            let caps = re.captures(value);
            match caps {
                Some(caps) => {
                    let height = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let unit = caps.get(2).unwrap().as_str();

                    match unit {
                        "cm" => height >= 150 && height <= 193,
                        "in" => height >= 59 && height <= 76,
                        _ => unreachable!(),
                    }
                }
                None => false,
            }
        },
        "hcl" => {
            let re = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            re.is_match(value)
        },
        "ecl" => {
            let re = regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            re.is_match(value)
        },
        "pid" => {
            let re = regex::Regex::new(r"^[0-9]{9}$").unwrap();
            re.is_match(value)
        },
        "cid" => true,
        _ => unreachable!(),
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut passports = Vec::new();
    let mut passport = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }

        for field in line.split(" ") {
            let mut iter = field.split(":");
            let key = iter.next().unwrap().to_string();
            let value = iter.next().unwrap().to_string();

            passport.insert(key, value);
        }
    }
    passports.push(passport);

    let required_keys = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
        // "cid",
    ];

    let answer_part_1 = passports.iter().filter(|passport|
        required_keys.iter().all(|key| passport.contains_key(key))
    ).count();

    println!("{}", answer_part_1);

    let answer_part_2 = passports.iter().filter(|passport|
        required_keys.iter().all(|key|
            match passport.get(key) {
                Some(value) => field_valid(key, value),
                None => false,
            }
        )
    ).count();

    println!("{}", answer_part_2);    
}
