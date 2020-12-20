use std::{collections::HashSet, io::BufRead};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Edge {
    UpperCW,
    UpperCCW,
    RightCW,
    RightCCW,
    LowerCW,
    LowerCCW,
    LeftCW,
    LeftCCW,
}

fn get_edge(tile: &Vec<Vec<char>>, edge: &Edge) -> Vec<char> {
    match edge {
        Edge::UpperCW => {
            tile.first().unwrap().iter().copied().collect::<Vec<_>>()
        }
        Edge::UpperCCW => {
            tile.first().unwrap().iter().rev().copied().collect::<Vec<_>>()
        }
        Edge::RightCW => {
            tile.iter().map(|row| row.last().unwrap()).copied().collect::<Vec<_>>()
        }
        Edge::RightCCW => {
            tile.iter().map(|row| row.last().unwrap()).rev().copied().collect::<Vec<_>>()
        }
        Edge::LowerCW => {
            tile.last().unwrap().iter().rev().copied().collect::<Vec<_>>()
        }
        Edge::LowerCCW => {
            tile.last().unwrap().iter().copied().collect::<Vec<_>>()
        }
        Edge::LeftCW => {
            tile.iter().map(|row| row.first().unwrap()).rev().copied().collect::<Vec<_>>()
        }
        Edge::LeftCCW => {
            tile.iter().map(|row| row.first().unwrap()).copied().collect::<Vec<_>>()
        }
    }
}

fn edges_match(tile_a: &Vec<Vec<char>>, edge_a: &Edge, tile_b: &Vec<Vec<char>>, edge_b: &Edge) -> bool {
    get_edge(tile_a, edge_a).iter().zip(get_edge(tile_b, edge_b).iter()).all(|(char_a, char_b)| char_a == char_b)
}

fn find_all_neighbors(tiles: &HashMap<u64, Vec<Vec<char>>>) -> HashMap<u64, Vec<u64>> {
    let mut neighbors = HashMap::new();

    let all_edges = vec![Edge::UpperCW, Edge::RightCW, Edge::LowerCW, Edge::LeftCW, Edge::UpperCCW, Edge::RightCCW, Edge::LowerCCW, Edge::LeftCCW];

    for (id_a, tile_a) in tiles.iter() {
        let mut tile_a_neighbors = Vec::new();

        for (id_b, tile_b) in tiles.iter() {
            if id_b == id_a {
                continue;
            }

            for i in 0..4 {
                for j in 0..8 {
                    if edges_match(tile_a, &all_edges[i], tile_b, &all_edges[j]) {
                        // tile_a_neighbors.insert(*id_b);
                        tile_a_neighbors.push(*id_b);
                    }
                }
            }
        }

        neighbors.insert(*id_a, tile_a_neighbors);
    }

    neighbors
}

fn display(tile: &Vec<Vec<char>>) {
    let output = tile.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
    println!("{}\n", output);
}

fn flip(tile: &Vec<Vec<char>>, num_flips: u32) -> Vec<Vec<char>> {
    if num_flips % 2 == 1 {
        tile.iter().rev().cloned().collect()
    } else {
        tile.clone()
    }
}

fn rotate(tile: &Vec<Vec<char>>, num_rotates: u32) -> Vec<Vec<char>> {
    let mut result = tile.clone();
    for _i in 0..num_rotates {
        let mut tmp = Vec::new();
        for j in 0..tile[0].len() {
            tmp.push(result.iter().map(|row| row[j]).rev().collect::<Vec<_>>());
        }

        result = tmp;
    }

    result
}

fn match_tile(tiles: &HashMap<u64, Vec<Vec<char>>>, tile_id: u64, neighbors: &Vec<u64>, new_row: bool) -> (u64, Vec<Vec<char>>) {
    let tile = &tiles[&tile_id];
    for neighbor_id in neighbors {
        let original_neighbor = &tiles[neighbor_id];
        for num_flips in 0..2 {
            for num_rotates in 0..4 {
                let neighbor = flip(&rotate(original_neighbor, num_rotates), num_flips);
                if new_row {
                    if edges_match(tile, &Edge::LowerCCW, &neighbor, &Edge::UpperCW) {
                        return (*neighbor_id, neighbor);
                    }
                } else {
                    if edges_match(tile, &Edge::RightCW, &neighbor, &Edge::LeftCCW) {
                        return (*neighbor_id, neighbor);
                    }
                }
            }
        }
    }

    dbg!(tile_id, neighbors);
    (0, Vec::new())
}

fn assemble_image(tiles: &HashMap<u64, Vec<Vec<char>>>, all_neighbors: &HashMap<u64, Vec<u64>>) -> Vec<Vec<char>> {
    let mut image = Vec::new();
    image.push(Vec::new());
    let mut atlas = Vec::new();
    atlas.push(Vec::new());
    let size = (tiles.len() as f32).sqrt().trunc() as usize;

    let all_edges = vec![Edge::UpperCW, Edge::RightCW, Edge::LowerCW, Edge::LeftCW, Edge::UpperCCW, Edge::RightCCW, Edge::LowerCCW, Edge::LeftCCW];

    let first_tile_neighbors = all_neighbors.iter().filter(|(_tile_id, neighbors)| {
        neighbors.len() == 2
    }).nth(0).unwrap();

    // display(&tiles[first_tile_neighbors.0]);
    // display(&rotate(&tiles[first_tile_neighbors.0], 1));

    for num_rotates in 0..4 {
        let first_tile = rotate(&tiles[first_tile_neighbors.0], num_rotates);
        let has_right_neighbor = all_edges.iter().any(|edge| {
            first_tile_neighbors.1.iter().any(|neighbor| {
                edges_match(&first_tile, &Edge::RightCW, &tiles[neighbor], edge)
            })
        });
        let has_lower_neighbor = all_edges.iter().any(|edge| {
            first_tile_neighbors.1.iter().any(|neighbor| {
                edges_match(&first_tile, &Edge::LowerCCW, &tiles[neighbor], edge)
            })
        });
        
        if has_right_neighbor && has_lower_neighbor {
            dbg!(first_tile_neighbors.0, num_rotates);
            image[0].push(first_tile);
            atlas[0].push(*first_tile_neighbors.0);
            break;
        }
    }

    for y in 0..size {
        for x in 0..size {
            if x == 0 && y == 0 {
                continue;
            }

            if x == 0 {
                // attach to tile above
                let tile_id = atlas[y - 1][x];
                let matching_tile = match_tile(tiles, tile_id, &all_neighbors[&tile_id], true);

                image.push(Vec::new());
                image[y].push(matching_tile.1);
                atlas.push(Vec::new());
                atlas[y].push(matching_tile.0);            
            } else {
                // attach to tile to the left
                let tile_id = atlas[y][x - 1];
                // dbg!(tile_id, y, x - 1, &all_neighbors);
                let matching_tile = match_tile(tiles, tile_id, &all_neighbors[&tile_id], false);

                image[y].push(matching_tile.1);
                atlas[y].push(matching_tile.0);            
            }
        }
    }

    dbg!(&atlas);

    Vec::new()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut tiles = HashMap::new();

    let re = regex::Regex::new(r"^Tile (\d+):$").unwrap();
    let mut iter = reader.lines();

    let id_line = iter.next().unwrap().unwrap();
    let caps = re.captures(&id_line).unwrap();
    let mut current_tile_id = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();

    let mut current_tile = Vec::new();

    while let Some(line) = iter.next() {
        let line = line.unwrap();
        if line.is_empty() {
            tiles.insert(current_tile_id, current_tile);

            let id_line = iter.next().unwrap().unwrap();
            let caps = re.captures(&id_line).unwrap();
            current_tile_id = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        
            current_tile = Vec::new();
        
            continue;
        }

        current_tile.push(line.chars().collect::<Vec<_>>());
    }
    tiles.insert(current_tile_id, current_tile);

    dbg!(tiles.len());

    // let current_tile = tiles.get(&2311).unwrap();
    // dbg!(get_edge(current_tile, &Edge::UpperCW), &Edge::UpperCW);
    // dbg!(get_edge(current_tile, &Edge::UpperCCW), &Edge::UpperCCW);
    // dbg!(get_edge(current_tile, &Edge::RightCW), &Edge::RightCW);
    // dbg!(get_edge(current_tile, &Edge::RightCCW), &Edge::RightCCW);
    // dbg!(get_edge(current_tile, &Edge::LowerCW), &Edge::LowerCW);
    // dbg!(get_edge(current_tile, &Edge::LowerCCW), &Edge::LowerCCW);
    // dbg!(get_edge(current_tile, &Edge::LeftCW), &Edge::LeftCW);
    // dbg!(get_edge(current_tile, &Edge::LeftCCW), &Edge::LeftCCW);   

    let all_neighbors = find_all_neighbors(&tiles);
    let answer_part_1: u64 = all_neighbors.iter().map(|(tile_id, neighbors)| {
        if neighbors.len() == 2 {
            *tile_id
        } else {
            1
        }
    }).product();

    println!("{}", answer_part_1);

    assemble_image(&tiles, &all_neighbors);
}
