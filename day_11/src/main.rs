use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn multipler(value: i64, times: i64, h: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(&result) = h.get(&(value, times)) {
        return result;
    }
    if times == 0 {
        return 1;
    }
    if value == 0{
        let result = multipler(1, times-1, h);
        h.insert((value, times), result);
        return result;
    }
    if count_digits(value) % 2 == 0{
        let (left, right) = split_in_half(value);
        let result = multipler(left, times-1, h) + multipler(right, times-1, h);
        h.insert((value, times), result);
        return result;
    }
    let result = multipler(value*2024, times-1, h);
    h.insert((value, times), result);
    result
}

fn split_in_half(n: i64) -> (i64, i64) {
    let s = n.to_string();
    let (left, right) = s.split_at(s.len() / 2);
    (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap())
}

fn count_digits(n: i64) -> usize {
    if n == 0 {
        1
    } else {
        (n.abs() as f64).log10().floor() as usize + 1
    }
}


fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut stones: Vec<i64> = Vec::new();

    let mut total_stones_25 = 0;
    let mut total_stones_75 = 0;

    let mut helper: HashMap<(i64, i64), i64> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        stones = line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
    }

    for s in stones {
        total_stones_25 += multipler(s, 25, &mut helper);
        total_stones_75 += multipler(s, 75, &mut helper);
    }

    println!("Total stone 25 blink: {}", total_stones_25);
    println!("Total stone 75 blink: {}", total_stones_75);

    Ok(())
}
