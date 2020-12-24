use std::{collections::HashSet, io::BufRead};

fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    let offsets = vec![(2, 0), (1, 1), (-1, 1), (-2, 0), (1, -1), (-1, -1)];
    offsets.iter().map(|offset| (x + offset.0, y + offset.1)).collect()
}

fn display(black_tiles: &HashSet<(i32, i32)>) {
    let min_x = black_tiles.iter().map(|tile| tile.0).min().unwrap() - 2;
    let max_x = black_tiles.iter().map(|tile| tile.0).max().unwrap() + 3;
    let min_y = black_tiles.iter().map(|tile| tile.1).min().unwrap() - 1;
    let max_y = black_tiles.iter().map(|tile| tile.1).max().unwrap() + 2;

    dbg!(min_y, max_y, min_x, max_x);
    for y in min_y..max_y {
        for x in min_x..max_x {
            if x.rem_euclid(2) == y.rem_euclid(2) {
                if black_tiles.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut black_tiles = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let mut x = 0;
        let mut y = 0;

        let mut iter = line.chars();
        while let Some(character) = iter.next() {
            match character {
                'e' => {
                    x += 2;
                }
                's' => {
                    y += 1;
                    match iter.next() {
                        Some('e') => {
                            x += 1; 
                        }
                        Some('w') => {
                            x -= 1;
                        }
                        _ => unreachable!(),
                    }
                }
                'w' => {
                    x -= 2;
                }
                'n' => {
                    y -= 1;
                    match iter.next() {
                        Some('e') => {
                            x += 1; 
                        }
                        Some('w') => {
                            x -= 1;
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        if black_tiles.contains(&(x, y)) {
            black_tiles.remove(&(x, y));
        } else {
            black_tiles.insert((x, y));
        }
    }

    println!("{}", black_tiles.len());

    for _ in 0..100 {
        let mut white_tiles = HashSet::new();
        let mut new_black_tiles = HashSet::new();

        for (x, y) in black_tiles.iter() {
            for (neighbor_x, neighbor_y) in neighbors(*x, *y) {
                if !black_tiles.contains(&(neighbor_x, neighbor_y)) {
                    white_tiles.insert((neighbor_x, neighbor_y));
                }
            }

            let black_neighbors = neighbors(*x, *y).iter().filter(|neighbor| black_tiles.contains(neighbor)).count();
            if black_neighbors == 1 || black_neighbors == 2 {
                new_black_tiles.insert((*x, *y));
            }
        }
        
        for (x, y) in white_tiles.iter() {
            let black_neighbors = neighbors(*x, *y).iter().filter(|neighbor| black_tiles.contains(neighbor)).count();
            if black_neighbors == 2 {
                new_black_tiles.insert((*x, *y));
            }
        }
        
        black_tiles = new_black_tiles;
    }

    println!("{}", black_tiles.len());
}
