use std::io::BufRead;

#[derive(Debug)]
enum Token {
    Number(u64),
    Add,
    Multiply,
    OpenParenthesis,
    CloseParenthesis,
}

fn evaluate_part1(tokens: &Vec<Token>, start: usize) -> (u64, usize) {
    let mut accumulator = 0;
    let mut last_operator = None;

    let mut current = start;
    while current < tokens.len() {
        match tokens[current] {
            Token::Number(number) => {
                match &last_operator {
                    Some(operator) => {
                        accumulator = match operator {
                            Token::Add => accumulator + number,
                            Token::Multiply => accumulator * number,
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        accumulator = number;
                    }
                }
                current += 1;
            }
            Token::Add => {
                last_operator = Some(Token::Add);
                current += 1;
            }
            Token::Multiply => {
                last_operator = Some(Token::Multiply);
                current += 1;
            }
            Token::OpenParenthesis => {
                // dbg!("recursion");
                let result = evaluate_part1(tokens, current + 1);
                let number = result.0;
                match &last_operator {
                    Some(operator) => {
                        accumulator = match operator {
                            Token::Add => accumulator + number,
                            Token::Multiply => accumulator * number,
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        accumulator = number;
                    }
                }
                current = result.1;
            }
            Token::CloseParenthesis => {
                current += 1;
                return (accumulator, current);
            }
        }
    }

    return (accumulator, current);
}

fn evaluate_part2(tokens: &Vec<Token>, start: usize) -> (u64, usize) {
    let mut accumulator = 0;
    let mut last_operator = None;

    let mut current = start;
    while current < tokens.len() {
        match tokens[current] {
            Token::Number(number) => {
                match &last_operator {
                    Some(operator) => {
                        accumulator = match operator {
                            Token::Add => accumulator + number,
                            Token::Multiply => {
                                let result = evaluate_part2(tokens, current);
                                current = result.1;
                                if let Token::CloseParenthesis = tokens[current - 1] {
                                    return (accumulator * result.0, current);
                                }
                                accumulator * result.0
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        accumulator = number;
                    }
                }
                current += 1;
            }
            Token::Add => {
                last_operator = Some(Token::Add);
                current += 1;
            }
            Token::Multiply => {
                last_operator = Some(Token::Multiply);
                current += 1;
            }
            Token::OpenParenthesis => {
                let result = evaluate_part2(tokens, current + 1);
                let number = result.0;
                match &last_operator {
                    Some(operator) => {
                        accumulator = match operator {
                            Token::Add => accumulator + number,
                            Token::Multiply => {
                                let result = evaluate_part2(tokens, current);
                                current = result.1;
                                if let Token::CloseParenthesis = tokens[current - 1] {
                                    return (accumulator * result.0, current);
                                }
                                accumulator * result.0
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        accumulator = number;
                    }
                }
                current = result.1;
            }
            Token::CloseParenthesis => {
                current += 1;
                return (accumulator, current);
            }
        }
    }

    return (accumulator, current);
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut answer_part1 = 0;
    let mut answer_part2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut tokens = vec![Token::OpenParenthesis];
        let mut current_token = "".to_string();
        for character in line.chars() {
            match character {
                '+' => {
                    tokens.push(Token::Add);
                }
                '*' => {
                    tokens.push(Token::Multiply);
                }
                '(' => {
                    tokens.push(Token::OpenParenthesis);
                }
                ')' => {
                    if current_token.len() > 0 {
                        tokens.push(Token::Number(current_token.parse().unwrap()));
                        current_token = "".to_string();
                    }

                    tokens.push(Token::CloseParenthesis);
                }
                ' ' => {
                    if current_token.len() > 0 {
                        tokens.push(Token::Number(current_token.parse().unwrap()));
                        current_token = "".to_string();
                    }
                }
                digit => {
                    current_token.push(digit);
                }
            }
        }

        if current_token.len() > 0 {
            tokens.push(Token::Number(current_token.parse().unwrap()));
            current_token = "".to_string();
        }

        tokens.push(Token::CloseParenthesis);

        let result = evaluate_part1(&tokens, 0);
        answer_part1 += result.0;

        let result = evaluate_part2(&tokens, 0);
        answer_part2 += result.0;
    }

    println!("{}", answer_part1);
    println!("{}", answer_part2);
}
