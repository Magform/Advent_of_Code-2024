use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn creates_antinodes(to_operate: Vec<Vec<char>>, mut results: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = to_operate.len();
    let width = to_operate[0].len();
    
    for y1 in 0..height {
        for x1 in 0..width {
            if to_operate[y1][x1] != '.' {
                for y2 in y1..height {
                    for x2 in 0..width {
                        if (x1 != x2 || y1 != y2) && to_operate[y1][x1] == to_operate[y2][x2] {
                            let dx = (x1 as isize - x2 as isize).abs();
                            let dy = (y1 as isize - y2 as isize).abs();

                            match (x1 < x2, y1 < y2) {
                                (true, true) => {
                                    update_results(&mut results, y1 as isize - dy, x1 as isize - dx);
                                    update_results(&mut results, y2 as isize + dy, x2 as isize + dx);
                                },
                                (true, false) => {
                                    update_results(&mut results, y1 as isize + dy, x1 as isize - dx);
                                    update_results(&mut results, y2 as isize - dy, x2 as isize + dx);
                                },
                                (false, true) => {
                                    update_results(&mut results, y1 as isize - dy, x1 as isize + dx);
                                    update_results(&mut results, y2 as isize + dy, x2 as isize - dx);
                                },
                                (false, false) => {
                                    update_results(&mut results, y1 as isize + dy, x1 as isize + dx);
                                    update_results(&mut results, y2 as isize - dy, x2 as isize - dx);
                                }
                            }
                        }
                    }
                }

            }
        }
    }
    results
}

fn creates_antinodes_new(to_operate: Vec<Vec<char>>, mut results: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = to_operate.len();
    let width = to_operate[0].len();
    
    for y1 in 0..height {
        for x1 in 0..width {
            if to_operate[y1][x1] != '.' {
                for y2 in 0..height {
                    for x2 in 0..width {
                        if (x1 != x2 || y1 != y2) && to_operate[y1][x1] == to_operate[y2][x2] {
                            update_results(&mut results, y1 as isize, x1 as isize);
                            let dx = (x1 as isize - x2 as isize).abs();
                            let dy = (y1 as isize - y2 as isize).abs();

                            match (x1 < x2, y1 < y2) {
                                (true, true) => {
                                    for a in 1..height{
                                        let a = a as isize;
                                        update_results(&mut results, y1 as isize - dy*a, x1 as isize - dx*a);
                                        update_results(&mut results, y2 as isize + dy*a, x2 as isize + dx*a);
                                    }
                                },
                                (true, false) => {
                                    for a in 1..height{
                                        let a = a as isize;
                                        update_results(&mut results, y1 as isize + dy*a, x1 as isize - dx*a);
                                        update_results(&mut results, y2 as isize - dy*a, x2 as isize + dx*a);
                                    }
                                },
                                (false, true) => {
                                    for a in 1..height{
                                        let a = a as isize;
                                        update_results(&mut results, y1 as isize - dy*a, x1 as isize + dx*a);
                                        update_results(&mut results, y2 as isize + dy*a, x2 as isize - dx*a);
                                    }
                                },
                                (false, false) => {
                                    for a in 1..height{
                                        let a = a as isize;
                                        update_results(&mut results, y1 as isize + dy*a, x1 as isize + dx*a);
                                        update_results(&mut results, y2 as isize - dy*a, x2 as isize - dx*a);
                                    }
                                }
                            }
                        }
                    }
                }

            }
        }
    }
    results
}

fn update_results(results: &mut Vec<Vec<char>>, y: isize, x: isize) {
    if y >= 0 && y < results.len() as isize && x >= 0 && x < results[0].len() as isize {
        results[y as usize][x as usize] = '#';
    }
}

fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut results = Vec::new();
    let mut to_operate = Vec::new();

    let mut total_antinodes_one = 0;
    let mut total_antinodes_two = 0;

    for line in reader.lines() {
        let line = line?;
        
        let mut r1 = Vec::new();
        let mut r2 = Vec::new();
    
        for c in line.chars() {
            r1.push(c);
            r2.push('.');
        }
    
        results.push(r2);
        to_operate.push(r1);
    }

    let results_one = creates_antinodes(to_operate.clone(), results.clone());

    let results_two = creates_antinodes_new(to_operate.clone(), results.clone());
    
    for a in 0..results.len(){
        for b in 0..results[a].len(){
            if results_one[a][b] == '#' {
                total_antinodes_one += 1;
            }
            if results_two[a][b] == '#' {
                total_antinodes_two += 1;
            }
        }
    }
    // Output the results
    println!("Total antinodes one: {}", total_antinodes_one);
    println!("Total antinodes two: {}", total_antinodes_two);

    Ok(())
}