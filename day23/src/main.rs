use std::{collections::HashMap, io::BufRead};

fn play_game_naive(mut cups: Vec<u32>, iterations: u32) -> Vec<u32> {
    let min_label = cups.iter().copied().min().unwrap();
    let max_label = cups.iter().copied().max().unwrap();

    let mut current_index = 0;
    let mut current_label = cups[current_index];
    for _ in 0..iterations {
        let mut picked_up_cups = Vec::new();
        let mut remaining_cups = Vec::new();
        for i in 0..cups.len() {
            if i >= 1 && i <= 3 {
                picked_up_cups.push(cups[(current_index + i) % cups.len()]);
            } else {
                remaining_cups.push(cups[(current_index + i) % cups.len()]);
            }
        }

        let mut destination_label = current_label - 1;
        while destination_label < min_label || picked_up_cups.contains(&destination_label) {
            if destination_label < min_label {
                destination_label = max_label;
            } else {
                destination_label -= 1;
            }
        }

        cups = Vec::new();
        for cup in remaining_cups {
            cups.push(cup);

            if cup == destination_label {
                for cup in &picked_up_cups {
                    cups.push(*cup);
                }
            }
        }
       
        current_index = (cups.iter().position(|cup| *cup == current_label).unwrap() + 1) % cups.len();
        current_label = cups[current_index];
    }

    cups
}

#[derive(Debug)]
struct Cup {
    label: u32,
    next: usize,
}

#[derive(Debug)]
struct CupRing {
    data: Vec<Cup>,
    lut: HashMap<u32, usize>,
}

impl CupRing {
    fn from(cups: Vec<u32>) -> CupRing {
        let mut lut = HashMap::new();
        let data = cups.iter().enumerate().map(|(i, cup)| {
            lut.insert(*cup, i);
            Cup {
                label: *cup,
                next: (i + 1) % cups.len(),
            }
        }).collect();

        CupRing {
            data,
            lut,
        }
    }

    fn next(&self, i: usize) -> usize {
        self.data[i].next
    }

    fn at(&self, i: usize) -> u32 {
        self.data[i].label
    }

    fn find(&self, label: u32) -> usize {
        self.lut[&label]
    }

    fn next_three_contain(&self, i: usize, label: u32) -> bool {
        let mut i = i;
        for _ in 0..3 {
            i = self.data[i].next;
            if self.data[i].label == label {
                return true;
            }
        }

        false
    }

    fn move_three(&mut self, source: usize, destination: usize) {
        let start = self.data[source].next;
        let end = self.data[self.data[start].next].next;

        self.data[source].next = self.data[end].next;
        self.data[end].next = self.data[destination].next;
        self.data[destination].next = start;
    }

    fn to_vec(&self) -> Vec<u32> {
        let mut vec = Vec::new();
        let mut i = self.find(1);
        loop {
            vec.push(self.data[i].label);

            i = self.data[i].next;

            if self.data[i].label == 1 {
                break;
            }
        }

        vec
    }
}

fn play_game_fast(cups: Vec<u32>, iterations: u32) -> Vec<u32> {
    let min_label = cups.iter().copied().min().unwrap();
    let max_label = cups.iter().copied().max().unwrap();

    let mut cup_ring = CupRing::from(cups);

    let mut current_index = 0;
    let mut current_label = cup_ring.at(current_index);
    for _ in 0..iterations {
        let mut destination_label = current_label - 1;
        while destination_label < min_label || cup_ring.next_three_contain(current_index, destination_label) {
            if destination_label < min_label {
                destination_label = max_label;
            } else {
                destination_label -= 1;
            }
        }

        let destination = cup_ring.find(destination_label);
        cup_ring.move_three(current_index, destination);
       
        current_index = cup_ring.next(current_index);
        current_label = cup_ring.at(current_index);
    }

    cup_ring.to_vec()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let original_cups = line.chars().map(|cup| cup.to_digit(10).unwrap()).collect::<Vec<_>>();

    let cups = original_cups.clone();
    let cups = play_game_naive(cups, 100);
    let cup_one_position = cups.iter().position(|cup| *cup == 1).unwrap();
    for i in 1..cups.len() {
        print!("{}", cups[(cup_one_position + i) % cups.len()]);
    }
    println!();


    let mut cups = original_cups.clone();
    let max_label = cups.iter().copied().max().unwrap();
    for i in (max_label + 1)..1_000_001 {
        cups.push(i);
    }

    let cups = play_game_fast(cups, 10_000_000);
    println!("{}", cups[1] as u64 * cups[2] as u64);
}
