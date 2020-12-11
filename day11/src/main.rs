use std::io::BufRead;

fn occupied_neighbors(waiting_area: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    let neighbors = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1,1)];

    let mut occupied = 0;
    for neighbor in neighbors.iter().map(|neighbor| (x + neighbor.0, y + neighbor.1)) {
        let on_grid = neighbor.0 >= 0 && neighbor.0 < waiting_area[0].len() as i32 && neighbor.1 >= 0 && neighbor.1 < waiting_area.len() as i32;
        
        if on_grid && waiting_area[neighbor.1 as usize][neighbor.0 as usize] == '#' {
            occupied += 1;
        }
    }

    occupied
}

fn occupied_directions(waiting_area: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    let neighbors = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1,1)];

    let mut occupied = 0;
    for (i, neighbor) in neighbors.iter().map(|neighbor| (x + neighbor.0, y + neighbor.1)).enumerate() {
        let mut tile = neighbor;
        let mut on_grid = tile.0 >= 0 && tile.0 < waiting_area[0].len() as i32 && tile.1 >= 0 && tile.1 < waiting_area.len() as i32;
        while on_grid {
            // dbg!(x, y, &tile);

            if waiting_area[tile.1 as usize][tile.0 as usize] == '#' {
                occupied += 1;
                break;
            }

            if waiting_area[tile.1 as usize][tile.0 as usize] == 'L' {
                break;
            }

            tile = (tile.0 + neighbors[i].0, tile.1 + neighbors[i].1);
            on_grid = tile.0 >= 0 && tile.0 < waiting_area[0].len() as i32 && tile.1 >= 0 && tile.1 < waiting_area.len() as i32;
        }
    }

    occupied
}

fn display(waiting_area: &Vec<Vec<char>>) {
    let output = waiting_area.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
    println!("{}\n", output);
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut original_waiting_area = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        original_waiting_area.push(line.chars().collect::<Vec<_>>());
    }

    // display(&waiting_area);

    let mut waiting_area = original_waiting_area.clone();
    loop {
        let mut changed = 0;    
        waiting_area = waiting_area.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, tile)| {
                match tile {
                    'L' => {
                        if occupied_neighbors(&waiting_area, x as i32, y as i32) == 0 {
                            changed += 1;
                            '#'
                        } else {
                            'L'
                        }
                    }
                    '#' => {
                        if occupied_neighbors(&waiting_area, x as i32, y as i32) >= 4 {
                            changed += 1;
                            'L'
                        } else {
                            '#'
                        }
                    }
                    '.' => '.',
                    _ => unreachable!(),
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        if changed == 0 {
            break;
        }
    }

    // display(&waiting_area);

    let total_occupied: usize = waiting_area.iter().map(|row| row.iter().filter(|tile| **tile == '#').count()).sum();
    println!("{}", total_occupied);

    let mut waiting_area = original_waiting_area.clone();
    loop {
        let mut changed = 0;    
        waiting_area = waiting_area.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, tile)| {
                match tile {
                    'L' => {
                        if occupied_directions(&waiting_area, x as i32, y as i32) == 0 {
                            changed += 1;
                            '#'
                        } else {
                            'L'
                        }
                    }
                    '#' => {
                        if occupied_directions(&waiting_area, x as i32, y as i32) >= 5 {
                            changed += 1;
                            'L'
                        } else {
                            '#'
                        }
                    }
                    '.' => '.',
                    _ => unreachable!(),
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        // display(&waiting_area);

        if changed == 0 {
            break;
        }
    }

    let total_occupied: usize = waiting_area.iter().map(|row| row.iter().filter(|tile| **tile == '#').count()).sum();
    println!("{}", total_occupied);
}
