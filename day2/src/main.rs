use std::path::Path;
use std::io::{Result, Lines, BufReader, BufRead};
use std::fs::File;

fn main() {
    let path = Path::new("input.txt");

    let file = match read_file(&path) {
        Err(why) => panic!("unable to open file: {}", why),
        Ok(file) => file,
    };

    let (enemy, player) = split_data(file);

    let score = predict_score(enemy.clone(), player.clone());
    let score_2 = predict_strategy(enemy.clone(), player.clone());

    println!("Score using predict_score: {}", score);
    println!("Score using predict_strategy: {}", score_2);
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<File> {
    match File::open(path.as_ref()) {
        Err(why) => return Err(why),
        Ok(file) => return Ok(file),
    };
}

fn read_lines(file: File) -> Result<Lines<BufReader<File>>> {
    Ok(BufReader::new(file).lines())
}

fn split_data(file: File) -> (Vec<char>, Vec<char>) {
    let mut first_row: Vec<char> = Vec::new();
    let mut second_row: Vec<char> = Vec::new();
    
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(shape) = line {
                for s in shape.chars() {
                    match s {
                        'A'..='C' => first_row.push(s),
                        'X'..='Z' => second_row.push(s),
                        _ => continue,
                    };
                }
            }
        }
    }

    return (first_row, second_row)
}

fn predict_score(enemy: Vec<char>, player: Vec<char>) -> i32 {
    let mut score = 0;

    for (pos, e) in enemy.iter().enumerate() {
        if *e == 'A' {
           match player[pos] {
                'X' => score += 4,
                'Y' => score += 8,
                'Z' => score += 3,
                _ => continue,
           }
        } else if *e == 'B' {
            match player[pos] {
                'X' => score += 1,
                'Y' => score += 5,
                'Z' => score += 9,
                _ => continue,
            }
        } else if *e == 'C' {
            match player[pos] {
                'X' => score += 7,
                'Y' => score += 2,
                'Z' => score += 6,
                _ => continue,
            }
        }
    }

    return score
}

fn predict_strategy(enemy: Vec<char>, player: Vec<char>) -> i32 {
    let mut score = 0;

    for (pos, e) in enemy.iter().enumerate() {
        if *e == 'A' {
           match player[pos] {
                'X' => score += 3,
                'Y' => score += 4,
                'Z' => score += 8,
                _ => continue,
           }
        } else if *e == 'B' {
            match player[pos] {
                'X' => score += 1,
                'Y' => score += 5,
                'Z' => score += 9,
                _ => continue,
            }
        } else if *e == 'C' {
            match player[pos] {
                'X' => score += 2,
                'Y' => score += 6,
                'Z' => score += 7,
                _ => continue,
            }
        }
    }

    return score
}
