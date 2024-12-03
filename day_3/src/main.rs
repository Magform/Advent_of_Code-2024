use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;


fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut total = 0;

    let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

    for line in reader.lines(){
        let line = line?;
        for valid in reg.find_iter(&line){
            let valid = valid.as_str();
            let pattern = Regex::new(r"mul\(|,|\)").unwrap();
            let v: Vec<&str> = pattern.split(valid).collect();
            total += v[1].parse::<i32>().unwrap() * v[2].parse::<i32>().unwrap();
        }
    }

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut total_enabling = 0;
    let mut active: bool = true;

    let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)|don't\(\)|do\(\)").unwrap();


    for line in reader.lines(){
        let line = line?;
        for valid in reg.find_iter(&line){
            let valid = valid.as_str();
            if valid == "don't()" {
                active = false;
            }else if valid == "do()"{
                active = true;
            }else{
                let pattern = Regex::new(r"mul\(|,|\)").unwrap();
                let v: Vec<&str> = pattern.split(valid).collect();
                if active{
                    total_enabling += v[1].parse::<i32>().unwrap() * v[2].parse::<i32>().unwrap();
                }
            }
        }
    }

    println!("Total: {}", total);
    println!("Total enabling: {}", total_enabling);

    Ok(())
}
