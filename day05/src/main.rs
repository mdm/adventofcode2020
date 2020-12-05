use std::io::BufRead;

fn find_seat_id(instructions: &str) -> u32 {
    let mut row_min = 0; // inclusive
    let mut row_max = 128; // exclusive
    let mut column_min = 0; // inclusive
    let mut column_max = 8; // exclusive

    for instruction in instructions.chars() {
        match instruction {
            'F' => {
                row_max = (row_min + row_max) >> 1;
            },
            'B' => {
                row_min = (row_min + row_max) >> 1;
            },
            'L' => {
                column_max = (column_min + column_max) >> 1;
            },
            'R' => {
                column_min = (column_min + column_max) >> 1;
            },
            _ => unreachable!(),
        }
    }
    
    row_min * 8 + column_min
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut seat_ids = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        seat_ids.push(find_seat_id(&line));
    }

    println!("{}", seat_ids.iter().max().unwrap());

    seat_ids.sort();
    for (seat_a, seat_b) in seat_ids.iter().zip(seat_ids.iter().skip(1)) {
        if seat_b - seat_a != 1 {
            println!("{}", seat_a + 1);
            break;
        }
    }
}
