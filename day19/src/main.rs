use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Terminal(char),
    Choices(Vec<Vec<usize>>)
}

fn parse(
    rules: &HashMap<usize, Rule>,
    starting_rule_id: usize,
    string: &str,
    starting_position: usize,
    rule_eight: usize,
    rule_eleven: usize
) -> Option<usize> {
    match starting_rule_id {
        8 => {
            let choice = vec![42; rule_eight];
            
            let mut new_starting_position = starting_position;
            let mut failure = false;
            for rule in choice {
                let result = parse(rules, rule, string, new_starting_position, rule_eight, rule_eleven);
                match result {
                    Some(end_postion) => {
                        new_starting_position = end_postion;
                    }
                    None => {
                        failure = true;
                        break;
                    }
                }
            }

            if failure {
                return None;
            } else {
                return Some(new_starting_position);
            }
        }
        11 => {
            let mut tmp = vec![31; rule_eleven];
            let mut choice = vec![42; rule_eleven];
            choice.append(&mut tmp);
        
            let mut new_starting_position = starting_position;
            let mut failure = false;
            for rule in choice {
                let result = parse(rules, rule, string, new_starting_position, rule_eight, rule_eleven);
                match result {
                    Some(end_postion) => {
                        new_starting_position = end_postion;
                    }
                    None => {
                        failure = true;
                        break;
                    }
                }
            }

            if failure {
                return None;
            } else {
                return Some(new_starting_position);
            }
        }
        _ => {}
    }

    match &rules[&starting_rule_id] {
        Rule::Terminal(target) => {
            match string.chars().nth(starting_position) {
                Some(character) => {
                    if character == *target {
                        return Some(starting_position + 1);
                    } else {
                        return None;
                    }
                }
                None => {
                    return None;
                }
            }
        }
        Rule::Choices(choices) => {
            for choice in choices {
                let mut new_starting_position = starting_position;
                let mut failure = false;
                for rule in choice {
                    let result = parse(rules, *rule, string, new_starting_position, rule_eight, rule_eleven);
                    match result {
                        Some(end_postion) => {
                            new_starting_position = end_postion;
                        }
                        None => {
                            failure = true;
                            break;
                        }
                    }
                }

                if !failure {
                    return Some(new_starting_position);
                }
            }
        }
    }

    None
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut rules = HashMap::new();
    let mut strings = Vec::new();

    let mut block = 0;
    let mut iter = reader.lines();
    while let Some(line) = iter.next() {
        let line = line.unwrap();
        if line.is_empty() {
            block += 1;
            continue;
        }

        match block {
            0 => {
                let id = line.split(": ").nth(0).unwrap().parse::<usize>().unwrap();
                let rhs = line.split(": ").nth(1).unwrap();
                if rhs == "\"a\"" || rhs == "\"b\"" {
                    rules.insert(id, Rule::Terminal(rhs.chars().nth(1).unwrap()));
                } else {
                    let choices = rhs.split(" | ").map(|choice| {
                        choice.split(' ').map(|rule| {
                            rule.parse::<usize>().unwrap()
                        }).collect::<Vec<_>>()
                    }).collect::<Vec<_>>();

                    rules.insert(id, Rule::Choices(choices));
                }
            }
            1 => {
                strings.push(line);
            }
            _ => unreachable!(),
        }
    }

    let answer_part1 = strings.iter().map(|string| {
        let result = parse(&rules, 0, string, 0, 1, 1);
        match result {
            Some(matched) => {
                if matched < string.len() {
                    0
                } else {
                    1
                }
            },
            None => 0,
        }
    }).sum::<usize>();

    println!("{}", answer_part1);

    let answer_part2 = strings.iter().map(|string| {
        for rule_eight in 0..string.len() {
            for rule_eleven in 0..string.len() {
                let result = parse(&rules, 0, string, 0, rule_eight + 1, rule_eleven + 1);
                if let Some(matched) = result {
                    if matched == string.len() {
                        return 1;
                    }
                }
            }
        }

        0
    }).sum::<usize>();

    println!("{}", answer_part2);
}
