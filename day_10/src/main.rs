use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn mover(map: &mut Vec<Vec<i32>>, y: usize, x: usize) -> i32 {
    if map[y][x] == 0 {
        map[y][x] = 1;
        return 1;
    }
    let mut to_return = 0;
    if x + 1 < map[0].len() && map[y][x] - 1 == map[y][x + 1] {
        to_return += mover(map, y, x + 1);
    }
    if x > 0 && map[y][x] - 1 == map[y][x - 1] {
        to_return += mover(map, y, x - 1);
    }
    if y + 1 < map.len() && map[y][x] - 1 == map[y + 1][x] {
        to_return += mover(map, y + 1, x);
    }
    if y > 0 && map[y][x] - 1 == map[y - 1][x] {
        to_return += mover(map, y - 1, x);
    }

    to_return
}

fn mover_rating(map: &Vec<Vec<i32>>, y: usize, x: usize) -> i32 {
    if map[y][x] == 0 {
        return 1;
    }
    let mut to_return = 0;
    if x + 1 < map[0].len() && map[y][x] - 1 == map[y][x + 1] {
        to_return += mover_rating(map, y, x + 1);
    }
    if x > 0 && map[y][x] - 1 == map[y][x - 1] {
        to_return += mover_rating(map, y, x - 1);
    }
    if y + 1 < map.len() && map[y][x] - 1 == map[y + 1][x] {
        to_return += mover_rating(map, y + 1, x);
    }
    if y > 0 && map[y][x] - 1 == map[y - 1][x] {
        to_return += mover_rating(map, y - 1, x);
    }

    to_return
}

fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut map: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<i32> = line.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        map.push(row);
    }

    let mut trailheads = 0;
    let mut rating = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 9 {
                let mut t_map = map.clone();
                rating += mover_rating(&t_map, y, x);
                trailheads += mover(&mut t_map, y, x);
            }
        }
    }


    println!("Total trailheads: {}", trailheads);
    println!("Total rating: {}", rating);

    Ok(())
}
