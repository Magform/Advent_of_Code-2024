use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn is_good(levels: &[i32]) -> bool {
    let new_levels = levels.to_vec();

    let new_diffs: Vec<i32> = new_levels.windows(2)
            .map(|w| w[1] - w[0])
            .collect();

    let valid_diffs = new_diffs.iter().all(|&d| (d.abs() >= 1) && (d.abs() <= 3));
    let increasing = new_diffs.iter().all(|&d| d > 0);
    let decreasing = new_diffs.iter().all(|&d| d < 0);

    valid_diffs && (increasing || decreasing)
}

fn can_be_made_good(levels: &[i32]) -> bool {
    levels.iter().enumerate().any(|(i, _)| {
        let mut new_levels = levels.to_vec();
        new_levels.remove(i);
        
        let new_diffs: Vec<i32> = new_levels.windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        
        let valid_diffs = new_diffs.iter().all(|&d| (d.abs() >= 1) && (d.abs() <= 3));
        let increasing = new_diffs.iter().all(|&d| d > 0);
        let decreasing = new_diffs.iter().all(|&d| d < 0);
        
        valid_diffs && (increasing || decreasing)
    })
}


fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut safe_report: i32 = 0;
    let mut safe_edited_report: i32 = 0;


    for line in reader.lines(){
        let line = line?;
        let numbers: Result<Vec<i32>, _> = line.split_whitespace().map(str::parse).collect();
        let n: &[i32] = &numbers.unwrap();
        if is_good(n) {
            safe_report += 1;
        }
    }

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    for line in reader.lines(){
        let line = line?;
        let numbers: Result<Vec<i32>, _> = line.split_whitespace().map(str::parse).collect();
        let n: &[i32] = &numbers.unwrap();
        if can_be_made_good(n) {
            safe_edited_report += 1;
        }
    }

    println!("Safe reports: {}", safe_report);
    println!("Safe edited reports: {}", safe_edited_report);

    Ok(())
}
