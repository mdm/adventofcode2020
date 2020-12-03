use std::io::BufRead;

#[derive(PartialEq)]
enum Tile {
    Ground,
    Tree,
}

fn check_slope(map: &Vec<Vec<Tile>>, step_x: usize, step_y: usize) -> u64 {
    let width = map[0].len();
    let height = map.len();

    let mut x = 0;
    let mut y = 0;

    let mut hits = 0;
    while y < height {
        if map[y][x % width] == Tile::Tree {
            hits += 1;
        }

        x += step_x;
        y += step_y;
    }

    hits
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let map = reader.lines().map(|line|
        line.unwrap().chars().map(|tile| match tile {
            '.' => Tile::Ground,
            '#' => Tile::Tree,
            _ => unreachable!(),
        }).collect::<Vec<_>>()
    ).collect::<Vec<_>>();


    println!("{}", check_slope(&map, 3, 1));

    let all_slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let all_hits_product: u64 = all_slopes.iter().map(|slope| check_slope(&map, slope.0, slope.1)).product();
    println!("{}", all_hits_product);
}
