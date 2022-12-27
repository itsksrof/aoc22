use std::path::Path;
use std::fs::File;
use std::io::{Result, Lines, BufReader, BufRead};

fn main() {
    let path = Path::new("input.txt");

    let file = match read_file(&path) {
        Err(why) => panic!("unable to open file: {}", why),
        Ok(file) => file,
    };

    let occurrences = match_splitted(file);
    let score = match_alphabet(occurrences);
   
    println!("Priority score: {}", score);
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

fn match_splitted(file: File) -> Vec<char> {
    let mut occurrences: Vec<char> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(string) = line {
                let mid = string.len() / 2;
                let (first, second) = string.split_at(mid);
                
                let mut last_occurrence: char = '\0';

                for c in first.chars() {
                    if c != last_occurrence {
                        match second.find(c) {
                            Some(_) => occurrences.push(c),
                            None => continue,
                        }

                        last_occurrence = c;
                    }

                    continue;
                }
            }
        }
    }

    return occurrences
}

fn match_alphabet(vec: Vec<char>) -> i32 {
    let mut score: i32 = 0;

    for c in vec {
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        match alphabet.find(c) {
            Some(pos) => score += pos as i32 + 1, 
            None => continue,
        };
    }

    return score
}
