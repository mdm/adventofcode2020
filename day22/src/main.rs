use std::{collections::{HashSet, VecDeque}, io::BufRead};

fn recursive_combat(mut decks: Vec<VecDeque<u32>>) -> (usize, Vec<VecDeque<u32>>) { // TODO: is it ok to pass history?
    let mut history = HashSet::new();

    while decks[0].len() > 0 && decks[1].len() > 0 {
        if history.contains(&decks) {
            return (0, decks);
        }
    
        history.insert(decks.clone());
    
        let cards = vec![decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap()];

        if decks[0].len() < cards[0] as usize || decks[1].len() < cards[1] as usize {
            if cards[0] == cards[1] {
                unreachable!();
            }

            if cards[0] > cards[1] {
                // decks[0] wins the round
                decks[0].push_back(cards[0]);
                decks[0].push_back(cards[1]);
            } else {
                // decks[1] wins the round
                decks[1].push_back(cards[1]);
                decks[1].push_back(cards[0]);
            }
        } else {
            let subdecks = decks.iter().enumerate().map(|(player, deck)| {
                deck.iter().take(cards[player] as usize).copied().collect::<VecDeque<_>>()
            }).collect::<Vec<_>>();
        
            let recursive = recursive_combat(subdecks);
            match recursive.0 {
                0 => {
                    // decks[0] wins the round
                    decks[0].push_back(cards[0]);
                    decks[0].push_back(cards[1]);
                }
                1 => {
                    // decks[1] wins the round
                    decks[1].push_back(cards[1]);
                    decks[1].push_back(cards[0]);
                }
                _ => unreachable!(),
            }
        }
    }

    if decks[0].len() > 0 {
        (0, decks)
    } else {
        (1, decks)
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut original_decks = vec![VecDeque::new(); 2];
    let mut player = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            player += 1;
            continue;
        }

        if let Ok(card) = line.parse::<u32>() {
            original_decks[player].push_back(card);
        }
    }


    let mut decks = original_decks.clone();
    while decks[0].len() > 0 && decks[1].len() > 0 {
        if decks[0].front() == decks[1].front() {
            unreachable!();
        }

        let card0 = decks[0].pop_front().unwrap();
        let card1 = decks[1].pop_front().unwrap();

        if card0 > card1 {
            // decks[0] wins the round
            decks[0].push_back(card0);
            decks[0].push_back(card1);
        } else {
            // decks[1] wins the round
            decks[1].push_back(card1);
            decks[1].push_back(card0);
        }
    }

    if decks[0].len() > 0 {
        // decks[0] wins the game
        let score = decks[0].iter().rev().zip(0..decks[0].len()).map(|(card, multiplier)| card * (multiplier as u32 + 1)).sum::<u32>();
        println!("{}", score);
    } else {
        // decks[1] wins the game
        let score = decks[1].iter().rev().zip(0..decks[1].len()).map(|(card, multiplier)| card * (multiplier as u32 + 1)).sum::<u32>();
        println!("{}", score);
    }


    let decks = original_decks.clone();
    let result = recursive_combat(decks);
    match result.0 {
        0 => {
            let score = result.1[0].iter().rev().zip(0..result.1[0].len()).map(|(card, multiplier)| card * (multiplier as u32 + 1)).sum::<u32>();
            println!("{}", score);
        }
        1 => {
            let score = result.1[1].iter().rev().zip(0..result.1[1].len()).map(|(card, multiplier)| card * (multiplier as u32 + 1)).sum::<u32>();
            println!("{}", score);
        }
        _ => unreachable!(),
    }   
}
