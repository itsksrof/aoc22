use std::fs::File;
use std::io::{Result, Lines, BufReader, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");

    let file = match read_file(&path) {
        Err(why) => panic!("unable to open file: {}", why),
        Ok(file) => file,
    };

    let mut sum_totals = sum_values(file);
    sum_totals.sort_by(|a, b| b.cmp(a));

    println!(
        "1: {}\n2: {}\n3: {}", 
        sum_totals[0], sum_totals[1], sum_totals[2],
    );

    println!(
        "Total: {}",
        sum_totals[0] + sum_totals[1] + sum_totals[2],
    );
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

fn sum_values(file: File) -> Vec<i32> {
    let mut sum: i32 = 0;
    let mut sum_totals: Vec<i32> = vec![];

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(num) = line {
                if num.is_empty() {
                    sum_totals.push(sum);
                    sum = 0;
                    continue;
                }

                sum += num.parse::<i32>().unwrap();
            }
        }
    }

    return sum_totals
}
