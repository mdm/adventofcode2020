use std::io::BufRead;

enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut ship1_x = 0;
    let mut ship1_y = 0;
    let mut ship2_x = 0;
    let mut ship2_y = 0;
    let mut ship_direction = Direction::East;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for line in reader.lines() {
        let line = line.unwrap();
        let action = line.chars().nth(0).unwrap();
        let value = line.chars().skip(1).collect::<String>().parse::<i32>().unwrap();

        match action {
            'N' => {
                ship1_y += value;
                waypoint_y += value;
            }
            'S' => {
                ship1_y -= value;
                waypoint_y -= value;
            }
            'E' => {
                ship1_x += value;
                waypoint_x += value;
            }
            'W' => {
                ship1_x -= value;
                waypoint_x -= value;
            }
            'L' => {
                match value {
                    90 => {
                        match ship_direction {
                            Direction::North => {
                                ship_direction = Direction::West;
                            }
                            Direction::South => {
                                ship_direction = Direction::East;
                            }
                            Direction::East => {
                                ship_direction = Direction::North;
                            }
                            Direction::West => {
                                ship_direction = Direction::South;
                            }       
                        }
                    },
                    180 => {
                        match ship_direction {
                            Direction::North => {
                                ship_direction = Direction::South;
                            }
                            Direction::South => {
                                ship_direction = Direction::North;
                            }
                            Direction::East => {
                                ship_direction = Direction::West;
                            }
                            Direction::West => {
                                ship_direction = Direction::East;
                            }       
                        }
                    },
                    270 => {
                        match ship_direction {
                            Direction::North => {
                                ship_direction = Direction::East;
                            }
                            Direction::South => {
                                ship_direction = Direction::West;
                            }
                            Direction::East => {
                                ship_direction = Direction::South;
                            }
                            Direction::West => {
                                ship_direction = Direction::North;
                            }       
                        }       
                    },
                    _ => {
                        dbg!(value);
                        unreachable!()
                    },
                }

                let radians = (value as f32) * std::f32::consts::PI / 180.0;
                let new_waypoint_x = waypoint_x * (radians.cos().trunc() as i32) + waypoint_y * -(radians.sin().trunc() as i32);
                let new_waypoint_y = waypoint_x * (radians.sin().trunc() as i32) + waypoint_y * (radians.cos().trunc() as i32);
                waypoint_x = new_waypoint_x;
                waypoint_y = new_waypoint_y;
            }
            'R' => {
                match value {
                    90 => {
                        match ship_direction {
                            Direction::North => {
                                ship_direction = Direction::East;
                            }
                            Direction::South => {
                                ship_direction = Direction::West;
                            }
                            Direction::East => {
                                ship_direction = Direction::South;
                            }
                            Direction::West => {
                                ship_direction = Direction::North;
                            }       
                        }
                    },
                    180 => {
                        match ship_direction {
                            Direction::North => {
                                ship_direction = Direction::South;
                            }
                            Direction::South => {
                                ship_direction = Direction::North;
                            }
                            Direction::East => {
                                ship_direction = Direction::West;
                            }
                            Direction::West => {
                                ship_direction = Direction::East;
                            }       
                        }
                    },
                    270 => {
                        match ship_direction {
                            Direction::North => {
                                ship_direction = Direction::West;
                            }
                            Direction::South => {
                                ship_direction = Direction::East;
                            }
                            Direction::East => {
                                ship_direction = Direction::North;
                            }
                            Direction::West => {
                                ship_direction = Direction::South;
                            }       
                        }
                    },
                    _ => unreachable!(),
                }

                let radians = (-value as f32) * std::f32::consts::PI / 180.0;
                let new_waypoint_x = waypoint_x * (radians.cos().trunc() as i32) + waypoint_y * -(radians.sin().trunc() as i32);
                let new_waypoint_y = waypoint_x * (radians.sin().trunc() as i32) + waypoint_y * (radians.cos().trunc() as i32);
                waypoint_x = new_waypoint_x;
                waypoint_y = new_waypoint_y;
            }
            'F' => {
                match ship_direction {
                    Direction::North => {
                        ship1_y += value;
                    }
                    Direction::South => {
                        ship1_y -= value;
                    }
                    Direction::East => {
                        ship1_x += value;
                    }
                    Direction::West => {
                        ship1_x -= value;
                    }       
                }

                ship2_x += value * waypoint_x;
                ship2_y += value * waypoint_y;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ship1_x.abs() + ship1_y.abs());
    println!("{}", ship2_x.abs() + ship2_y.abs());
}
