use std::{collections::HashSet, io::BufRead};

#[derive(Debug)]
struct Bag {
    gold: bool,
    color: String,
    contents: Vec<(u32, String)>
}

impl Bag {
    fn from(input: &str) -> Bag {
        let mut iter = input.split(' ');
        let color = format!("{} {}", iter.next().unwrap(), iter.next().unwrap());
        iter.next();
        iter.next();
        let mut contents= Vec::new();
        while let Some(bag_count) = iter.next() {
            if bag_count == "no" {
                break;
            }

            let parsed_bag_count = bag_count.parse::<u32>().unwrap();
            let bag_color = format!("{} {}", iter.next().unwrap(), iter.next().unwrap());
            contents.push((parsed_bag_count, bag_color));
            iter.next();
        }        

        Bag {
            gold: false,
            color,
            contents,
        }
    }

    fn contains(&self, color: &str) -> bool {
        self.contents.iter().filter(|bag| bag.1 == color).count() > 0
    }
}

fn count_contents(bags: &Vec<Bag>, start: &str) -> u32 {
    let bag = bags.iter().filter(|bag| bag.color == start).next().unwrap();

    bag.contents.iter().map(|bag| {
        bag.0 * (count_contents(bags, &bag.1) + 1)
    }).sum()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut bags = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        bags.push(Bag::from(&line));
    }

    let mut target_count = 0;
    let mut target_colors = HashSet::new();
    target_colors.insert("shiny gold".to_string());
    while target_colors.len() > target_count {
        target_count = target_colors.len();

        for bag in bags.iter_mut() {
            let mut new_target_colors = HashSet::new();
            for color in &target_colors {
                if bag.gold == false && bag.contains(color) {
                    bag.gold = true;
                    new_target_colors.insert(bag.color.clone());
                }
            }

            target_colors.extend(new_target_colors);
        }
    }

    println!("{}", bags.iter().filter(|bag| bag.gold).count());
    println!("{}", count_contents(&bags, "shiny gold"));
}
