use std::io::BufRead;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut answer_part_1 = 0;
    let mut answer_part_2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let re = regex::Regex::new(r"^(\d+)-(\d+) (\w): (.+)$").unwrap();
        let caps = re.captures(&line).unwrap();

        let pos_1 = caps.get(1).unwrap().as_str();
        let pos_2 = caps.get(2).unwrap().as_str();
        let min = pos_1.parse::<u32>().unwrap();
        let max = pos_2.parse::<u32>().unwrap();
        let focus_char = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();

        let mut char_count = 0;
        for character in password.chars() {
            if String::from(character) == focus_char {
                char_count += 1;
            }
        }

        if char_count >= min && char_count <= max {
            answer_part_1 += 1;
        }

        if (String::from(password.chars().nth(min as usize - 1).unwrap()) == focus_char && String::from(password.chars().nth(max as usize - 1).unwrap()) != focus_char)
        || (String::from(password.chars().nth(min as usize - 1).unwrap()) != focus_char && String::from(password.chars().nth(max as usize - 1).unwrap()) == focus_char) {
            answer_part_2 += 1;
        }
    }

    println!("{}", answer_part_1);
    println!("{}", answer_part_2);
}