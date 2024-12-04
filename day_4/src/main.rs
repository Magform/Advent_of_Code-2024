use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn xmas(location: Vec<Vec<char>>) -> i32 {
    let mut to_return = 0;

    let pos = [
        (0, 1),  // Right
        (-1, 1), // Up-right
        (-1, 0), // Up
        (-1, -1),// Up-left
        (0, -1), // Left
        (1, -1), // Down-left
        (1, 0),  // Down
        (1, 1),  // Down-right
    ];

    let to_what = ['X', 'M', 'A', 'S'];

    for v in 0..location.len() {
        for l in 0..location[v].len() {
            if location[v][l] == 'X' {
                for &(dx, dy) in &pos {
                    if let (Some(c1), Some(c2), Some(c3)) = (
                        location.get((v as isize + dx) as usize).and_then(|row| row.get((l as isize + dy) as usize)),
                        location.get((v as isize + 2 * dx) as usize).and_then(|row| row.get((l as isize + 2 * dy) as usize)),
                        location.get((v as isize + 3 * dx) as usize).and_then(|row| row.get((l as isize + 3 * dy) as usize)),
                    ) {
                        if [*c1, *c2, *c3] == [to_what[1], to_what[2], to_what[3]] {
                            to_return += 1;
                        }
                    }
                }
            }
        }
    }
    to_return
}

fn x_mas(location: Vec<Vec<char>>) -> i32 {
    let mut to_return = 0;

    let to_what = ['X', 'M', 'A', 'S'];

    for v in 0..location.len() {
        for l in 0..location[v].len() {
            if location[v][l] == 'A' {
                if let (Some(c1), Some(c2), Some(c3), Some(c4)) = (
                    location.get((v as isize - 1) as usize).and_then(|row| row.get((l as isize + 1) as usize)), // Up-right
                    location.get((v as isize - 1) as usize).and_then(|row| row.get((l as isize - 1) as usize)), // Up-left
                    location.get((v as isize + 1) as usize).and_then(|row| row.get((l as isize - 1) as usize)), // Down-left
                    location.get((v as isize + 1) as usize).and_then(|row| row.get((l as isize + 1) as usize)), // Down-right
                ) {
                    if [*c1, *c2, *c3, *c4] == [to_what[1], to_what[1],to_what[3], to_what[3]] ||
                    [*c1, *c2, *c3, *c4] == [to_what[3], to_what[1],to_what[1], to_what[3]] ||
                    [*c1, *c2, *c3, *c4] == [to_what[3], to_what[3],to_what[1], to_what[1]] ||
                    [*c1, *c2, *c3, *c4] == [to_what[1], to_what[3],to_what[3], to_what[1]]{
                        to_return += 1;
                    }
                }
                
            }
        }
    }
    to_return
}

fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        vec.push(line.chars().collect());
    }

    let total_xmas = xmas(vec.clone());
    let total_x_mas = x_mas(vec);

    println!("{}", total_xmas);
    println!("{}", total_x_mas);

    Ok(())
}
