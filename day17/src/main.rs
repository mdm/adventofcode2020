use std::{collections::HashSet, io::BufRead};

fn display(active_cubes: &HashSet<(i32, i32, i32, i32)>) {
    let min_x = active_cubes.iter().map(|(x, _y, _z, _w)| *x).min().unwrap();
    let max_x = active_cubes.iter().map(|(x, _y, _z, _w)| *x).max().unwrap() + 1;
    let min_y = active_cubes.iter().map(|(_x, y, _z, _w)| *y).min().unwrap();
    let max_y = active_cubes.iter().map(|(_x, y, _z, _w)| *y).max().unwrap() + 1;
    let min_z = active_cubes.iter().map(|(_x, _y, z, _w)| *z).min().unwrap();
    let max_z = active_cubes.iter().map(|(_x, _y, z, _w)| *z).max().unwrap() + 1;
    let min_w = active_cubes.iter().map(|(_x, _y, _z, w)| *w).min().unwrap();
    let max_w = active_cubes.iter().map(|(_x, _y, _z, w)| *w).max().unwrap() + 1;

    for w in min_w..max_w {
        for z in min_z..max_z {
            println!("z={}, w={}", z, w);
            for y in min_y..max_y {
                for x in min_x..max_x {
                    if active_cubes.contains(&(x, y, z, w)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn neighbors(x: i32, y: i32, z: i32, w: i32, subspace: bool) -> Vec<(i32, i32, i32, i32)> {
    let mut result = Vec::new();

    for offset_w in -1..2 {
        for offset_z in -1..2 {
            for offset_y in -1..2 {
                for offset_x in -1..2 {
                    if subspace {
                        if offset_x == 0 && offset_y == 0 && offset_z == 0 {
                            continue;
                        }
                        result.push((x + offset_x, y + offset_y, z + offset_z, w));                        
                    } else {
                        if offset_x == 0 && offset_y == 0 && offset_z == 0 && offset_w == 0 {
                            continue;
                        }
                        result.push((x + offset_x, y + offset_y, z + offset_z, w + offset_w));
                    }                    
                }
            }
        }

        if subspace {
            break;
        }
    }

    result
}

fn count_active_neighbors(active_cubes: &HashSet<(i32, i32, i32, i32)>, x: i32, y: i32, z: i32, w: i32, subspace: bool) -> u32 {
    neighbors(x, y, z, w, subspace).iter().map(|(neighbor_x, neighbor_y, neighbor_z, neighbor_w)| {
        if active_cubes.contains(&(*neighbor_x, *neighbor_y, *neighbor_z, *neighbor_w)) {
            1
        } else {
            0
        }
    }).sum()
}

fn step(active_cubes: &HashSet<(i32, i32, i32, i32)>, subspace: bool) -> HashSet<(i32, i32, i32, i32)> {
    let min_x = active_cubes.iter().map(|(x, _y, _z, _w)| *x).min().unwrap() - 1;
    let max_x = active_cubes.iter().map(|(x, _y, _z, _w)| *x).max().unwrap() + 2;
    let min_y = active_cubes.iter().map(|(_x, y, _z, _w)| *y).min().unwrap() - 1;
    let max_y = active_cubes.iter().map(|(_x, y, _z, _w)| *y).max().unwrap() + 2;
    let min_z = active_cubes.iter().map(|(_x, _y, z, _w)| *z).min().unwrap() - 1;
    let max_z = active_cubes.iter().map(|(_x, _y, z, _w)| *z).max().unwrap() + 2;
    let mut min_w = active_cubes.iter().map(|(_x, _y, _z, w)| *w).min().unwrap() - 1;
    let mut max_w = active_cubes.iter().map(|(_x, _y, _z, w)| *w).max().unwrap() + 2;

    if subspace {
        min_w = 0;
        max_w = 1;
    }

    let mut new_active_cubes = HashSet::new();

    for w in min_w..max_w {
        for z in min_z..max_z {
            for y in min_y..max_y {
                for x in min_x..max_x {
                    if active_cubes.contains(&(x, y, z, w)) {
                        let active_neighbors: u32 = count_active_neighbors(active_cubes, x, y, z, w, subspace);
                        if active_neighbors == 2 || active_neighbors == 3 {
                            new_active_cubes.insert((x, y, z, w));
                        }
                    } else {
                        let active_neighbors: u32 = count_active_neighbors(active_cubes, x, y, z, w, subspace);
                        if active_neighbors == 3 {
                            new_active_cubes.insert((x, y, z, w));
                        }                    
                    }
                }
            }
        }
    }

    new_active_cubes
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut inital_state = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        inital_state.push(line.chars().collect::<Vec<_>>());
    }

    let mut active_cubes = HashSet::new();
    for (y, row) in inital_state.iter().enumerate() {
        for (x, cube) in row.iter().enumerate() {
            if *cube == '#' {
                active_cubes.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    let cycles = 6;
    
    let mut active_cubes_part1 = active_cubes.clone();
    // display(&active_cubes_part1);
    
    for cycle in 0..cycles {
        active_cubes_part1 = step(&active_cubes_part1, true);
        // println!("After {} cycle(s)", cycle + 1);
        // display(&active_cubes_part1);
    }

    println!("{}", active_cubes_part1.len());
    
    let mut active_cubes_part2 = active_cubes.clone();
    // display(&active_cubes_part2);
    
    for cycle in 0..cycles {
        active_cubes_part2 = step(&active_cubes_part2, false);
        // println!("After {} cycle(s)", cycle + 1);
        // display(&active_cubes_part2);
    }

    println!("{}", active_cubes_part2.len());
}
