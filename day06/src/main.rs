use std::{io::BufRead};
use std::collections::HashSet;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let all_chars: HashSet<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut groups_one = Vec::new();
    let mut groups_all = Vec::new();
    let mut group_one = HashSet::new();
    let mut group_all = all_chars.clone();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            groups_one.push(group_one);
            groups_all.push(group_all);

            group_one = HashSet::new();
            group_all = all_chars.clone();

            continue;
        }

        let group = line.chars().collect();
        group_one = group_one.union(&group).copied().collect();
        group_all = group_all.intersection(&group).copied().collect();
    }
    groups_one.push(group_one);
    groups_all.push(group_all);

    let answer_part_1: usize = groups_one.iter().map(|group| group.len()).sum();
    println!("{}", answer_part_1);

    let answer_part_2: usize = groups_all.iter().map(|group| group.len()).sum();
    println!("{}", answer_part_2);
}
