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

fn flip(tile: &Vec<Vec<char>>, flip_type: u32) -> Vec<Vec<char>> {
    match flip_type {
        0 => tile.clone(),
        1 => tile.iter().map(|row| row.iter().rev().copied().collect()).collect(),
        2 => tile.iter().rev().cloned().collect(),
        _ => unreachable!(),
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

fn match_tile(tiles: &HashMap<u64, Vec<Vec<char>>>, tile: &Vec<Vec<char>>, neighbors: &Vec<u64>, new_row: bool) -> (u64, Vec<Vec<char>>) {
    for neighbor_id in neighbors {
        let original_neighbor = &tiles[neighbor_id];
        for flip_type in 0..3 {
            for num_rotates in 0..4 {
                let neighbor = flip(&rotate(original_neighbor, num_rotates), flip_type);
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

    unreachable!()
}

fn crop_tiles(image: &Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<Vec<Vec<char>>>> {
    image.iter().map(|image_row| {
        image_row.iter().map(|tile| {
            tile.iter().skip(1).take(tile.len() - 2).map(|tile_row| {
                tile_row.iter().skip(1).take(tile_row.len() - 2).copied().collect()
            }).collect()
        }).collect()
    }).collect()
}

fn stitch_tiles(image: &Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<char>> {
    image.iter().fold(Vec::new(), |mut stitched, image_row| {
        let rows = image_row.iter().fold(Vec::new(), |mut stitched, tile| {
            if stitched.len() == 0 {
                for _ in 0..tile.len() {
                    stitched.push(Vec::new());
                }
            }

            for (i, tile_row) in tile.iter().enumerate() {
                for character in tile_row.iter() {
                    stitched[i].push(*character);
                }
            }

            stitched
        });

        for row in rows {
            stitched.push(row);
        }

        stitched
    })    
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
                let matching_tile = match_tile(tiles, &image[y - 1][x], &all_neighbors[&tile_id], true);

                image.push(Vec::new());
                image[y].push(matching_tile.1);
                atlas.push(Vec::new());
                atlas[y].push(matching_tile.0);            
            } else {
                // attach to tile to the left
                let tile_id = atlas[y][x - 1];
                let matching_tile = match_tile(tiles, &image[y][x - 1], &all_neighbors[&tile_id], false);

                image[y].push(matching_tile.1);
                atlas[y].push(matching_tile.0);            
            }
        }
    }

    let cropped = crop_tiles(&image);
    let stitched = stitch_tiles(&cropped);

    stitched
}

fn find_sea_monsters(image: Vec<Vec<char>>) -> u64 {
    let mut sea_monster = Vec::new();
    sea_monster.push("                  # ".chars().collect::<Vec<_>>());
    sea_monster.push("#    ##    ##    ###".chars().collect::<Vec<_>>());
    sea_monster.push(" #  #  #  #  #  #   ".chars().collect::<Vec<_>>());
    let sea_monster_size = sea_monster.iter().map(|row| {
        row.iter().filter(|character| **character == '#').count() as u64
    }).sum::<u64>();

    let mut count = 0;
    for image_y in 0..(image.len() - sea_monster.len()) {
        for image_x in 0..(image[0].len() - sea_monster[0].len()) {
            let mut found = true;
            for sea_monster_y in 0..sea_monster.len() {
                for sea_monster_x in 0..sea_monster[0].len() {
                    let x = image_x + sea_monster_x;
                    let y = image_y + sea_monster_y;

                    if sea_monster[sea_monster_y][sea_monster_x] == '#' && image[y][x] != '#' {
                        found = false;
                        break;
                    }
                }

                if !found {
                    break;
                }
            }

            if found {
                count += sea_monster_size;
            }
        }
    }

    count
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


    let all_neighbors = find_all_neighbors(&tiles);
    let answer_part_1: u64 = all_neighbors.iter().map(|(tile_id, neighbors)| {
        if neighbors.len() == 2 {
            *tile_id
        } else {
            1
        }
    }).product();

    println!("{}", answer_part_1);

    let original_image = assemble_image(&tiles, &all_neighbors);
    let wave_count = original_image.iter().map(|row| {
        row.iter().filter(|character| **character == '#').count() as u64
    }).sum::<u64>();

    let mut found = false;
    for flip_type in 0..3 {
        for num_rotates in 0..4 {
            let image = flip(&rotate(&original_image, num_rotates), flip_type);
            let sea_monsters = find_sea_monsters(image);

            if sea_monsters > 0 {
                println!("{}", wave_count - sea_monsters);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }
}
