use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn is_good(values: Vec<i128>, result: i128) -> bool {
    if values.len() == 1 {
        return values[0] == result;
    }

    let mut values_mul = values.clone();
    values_mul[1] = values_mul[0] * values_mul[1];
    values_mul.remove(0);

    let mut values_add = values.clone();
    values_add[1] = values_add[0] + values_add[1];
    values_add.remove(0);

    is_good(values_mul, result) || is_good(values_add, result)
}

fn is_good_new(values: Vec<i128>, result: i128) -> bool {
    if values[0] > result {
        return false;
    }
    if values.len() == 1 {
        return values[0] == result;
    }

    let mut values_mul = values.clone();
    if let Some(new_value) = values_mul[0].checked_mul(values_mul[1]) {
        values_mul[1] = new_value;
    } else {
        return false;
    }
    values_mul.remove(0);

    let mut values_add = values.clone();
    values_add[1] = values_add[0] + values_add[1];
    values_add.remove(0);

    let mut values_con = values.clone();
    if let Some(concat_value) = concatenate_numbers(values_con[0], values_con[1]) {
        values_con[1] = concat_value;
    } else {
        return false;
    }
    values_con.remove(0);

    is_good_new(values_mul, result) || is_good_new(values_add, result) || is_good_new(values_con, result)
}

fn concatenate_numbers(a: i128, b: i128) -> Option<i128> {
    let a_str = a.to_string();
    let b_str = b.to_string();
    let concatenated = a_str + &b_str;
    concatenated.parse::<i128>().ok()
}

fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut results = Vec::new();
    let mut to_operate = Vec::new();

    let mut total_good = 0;
    let mut total_good_new = 0;

    for line in reader.lines() {
        let line = line?;
        let mut v = line.split(":");
        if let (Some(r), Some(other)) = (v.next(), v.next()) {
            if let Ok(r) = r.parse::<i128>() {
                results.push(r);
            }
            let o_l = other.split(" ");
            let mut to_push = Vec::new();
            for o in o_l {
                match o.parse::<i128>() {
                    Ok(num) => to_push.push(num),
                    Err(_) => {}
                }
            }

            to_operate.push(to_push);
        }
    }

    for n in 0..results.len() {
        if is_good(to_operate[n].clone(), results[n]) {
            total_good += results[n];
        }
        if is_good_new(to_operate[n].clone(), results[n]) {
            total_good_new += results[n];
        }
    }

    // Output the results
    println!("Total good: {}", total_good);
    println!("Total good with concatenation: {}", total_good_new);

    Ok(())
}