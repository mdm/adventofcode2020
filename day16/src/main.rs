use std::{collections::{HashMap, HashSet}, io::BufRead};

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut block = 0;
    let re = regex::Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut fields = HashMap::new();
    let mut my_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();

    let mut iter = reader.lines();
    while let Some(line) = iter.next() {
        let line = line.unwrap();
        if line.is_empty() {
            iter.next();
            block += 1;
            continue;
        }

        match block {
            0 => {
                let mut ranges = Vec::new();

                let caps = re.captures(&line).unwrap();
                let name = caps.get(1).unwrap().as_str().to_string();
                let start1 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let end1 = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
                let start2 = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
                let end2 = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

                ranges.push((start1, end1));
                ranges.push((start2, end2));

                fields.insert(name, ranges);
            }
            1 => {
                my_ticket = line.split(',').map(|number| number.parse::<u32>().unwrap()).collect::<Vec<_>>();
            }
            2 => {
                let ticket = line.split(',').map(|number| number.parse::<u32>().unwrap()).collect::<Vec<_>>();
                nearby_tickets.push(ticket);
            }
            _ => unreachable!(),
        }
    }

    let scanning_error_rate = nearby_tickets.iter().map(|ticket| {
        ticket.iter().map(|value| {
            let valid = fields.values().any(|ranges| {
                ranges.iter().any(|range| {
                    *value >= range.0 && *value <= range.1
                })
            });

            if valid {
                0
            } else {
                *value
            }
        }).sum::<u32>()
    }).sum::<u32>();

    println!("{}", scanning_error_rate);

    let valid_nearby_tickets = nearby_tickets.into_iter().filter(|ticket| {
        ticket.iter().all(|value| {
            fields.values().any(|ranges| {
                ranges.iter().any(|range| {
                    *value >= range.0 && *value <= range.1
                })
            })
        })
    }).collect::<Vec<_>>();

    let field_choices = valid_nearby_tickets.iter().map(|ticket| {
        ticket.iter().map(|value| {
            fields.iter().filter_map(move |(name, ranges)| {
                if ranges.iter().any(|range| {
                    value >= &range.0 && value <= &range.1
                }) {
                    Some(name)
                } else {
                    None
                }
            }).collect::<HashSet<_>>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut assignments = my_ticket.iter().enumerate().map(|(i, value)| {
        let choices = field_choices.iter().map(|ticket| {
            ticket[i].clone()
        }).fold(None, |choices_so_far: Option<HashSet<&String>>, new_choices| {
            match choices_so_far {
                Some(old_choices) => Some(old_choices.intersection(&new_choices).cloned().collect::<HashSet<_>>()),
                None => Some(new_choices),
            }
        }).unwrap();

        (choices, value)
    }).collect::<Vec<_>>();

    while assignments.iter().any(|assignment| assignment.0.len() > 1) {
        assignments = assignments.iter().map(|assignment| {
            if assignment.0.len() == 1 {
                return assignment.clone();
            }
            
            let choices = assignment.0.iter().cloned().filter(|choice| {
                assignments.iter().all(|other_assignment| {
                    !other_assignment.0.contains(choice) || other_assignment.0.len() > 1
                })
            }).collect::<HashSet<_>>();
            (choices, assignment.1)
        }).collect::<Vec<_>>();
    }

    let proof: u64 = assignments.iter().filter_map(|assignment| {
        if assignment.0.iter().next().unwrap().starts_with("departure") {
            Some(*assignment.1 as u64)
        } else {
            None
        }
    }).product();
    println!("{}", proof);
}
