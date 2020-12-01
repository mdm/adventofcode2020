use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    let numbers = reader.lines().map(|line|
        line.unwrap().parse::<u32>().unwrap()
    ).collect::<Vec<_>>();
    // dbg!(&numbers);

    for number_a in numbers.iter() {
        for number_b in numbers.iter() {
            if number_b < number_a {
                continue;
            }

            if number_a + number_b == 2020 {
                // dbg!(number_a, number_b);
                println!("{}", number_a * number_b);
            }
        }
    }

    for number_a in numbers.iter() {
        for number_b in numbers.iter() {
            if number_b < number_a {
                continue;
            }

            for number_c in numbers.iter() {
                if number_c < number_b {
                    continue;
                }
    
                if number_a + number_b + number_c == 2020 {
                    // dbg!(number_a, number_b, number_c);
                    println!("{}", number_a * number_b * number_c);
                }
            }
        }
    }

    Ok(())
}
