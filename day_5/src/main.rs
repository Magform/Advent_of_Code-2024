use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn check_valid(v: Vec<i32>, r: HashMap<i32, Vec<i32>>) -> bool {
    for n_value in 0..v.len() {
        if let Some(values) = r.get(&v[n_value]) {
            for &n in values {
                for a in 0..n_value {
                    if n == v[a] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn set_valid(mut v: Vec<i32>, r: HashMap<i32, Vec<i32>>) -> Vec<i32> {
    for n_value in 0..v.len() {
        if let Some(values) = r.get(&v[n_value]) {
            for &n in values {
                for a in 0..n_value {
                    if n == v[a] {
                        let tmp = v[n_value];
                        v[n_value] = v[a];
                        v[a] = tmp;
                    }
                }
            }
        }
    }
    if !check_valid(v.clone(), r.clone()) {
        v = set_valid(v.clone(), r.clone());
    }

    return v;
}

fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut rules = HashMap::new();
    let mut poss = Vec::new();

    let mut is_rules = true;

    let mut total_mid = 0;
    let mut edited_mid = 0;


    for line in reader.lines() {
        let line = line?;
        if line == ""{
            is_rules = false;
            continue;
        }
        if is_rules{
            let mut v = line.split("|");
            if let (Some(key), Some(value)) = (v.next(), v.next()) {
                if let (Ok(key), Ok(value)) = (key.parse::<i32>(), value.parse::<i32>()) {
                    rules.entry(key).or_insert_with(Vec::new).push(value);
                }
            }
        }else{
            let v = line.split(",");
            let mut tem = Vec::new();
            for value in v{
                if let Ok(n) = value.parse::<i32>() {
                    tem.push(n);
                }
            }
            poss.push(tem);

        }
    }

    for v in poss {
        if v.is_empty() {
            continue;
        } 
        if !check_valid(v.clone(), rules.clone()) {
            edited_mid += set_valid(v.clone(), rules.clone())[v.len() / 2];
            continue;
        }
        total_mid += v[v.len() / 2];
    }


    println!("Total mid: {}", total_mid);
    println!("Edited mid: {}", edited_mid);

    Ok(())
}
