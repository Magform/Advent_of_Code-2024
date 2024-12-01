use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines(){
        let line = line?;

        list1.push(line.split_whitespace().nth(0).unwrap().parse::<i32>().unwrap());
        list2.push(line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut map1: HashMap<i32, i32> = HashMap::new();
    let mut map2: HashMap<i32, i32> = HashMap::new();

    let mut total_difference: i32 = 0;

    for n in 0..list1.len(){
        if list1[n]>list2[n]{
            total_difference += list1[n] - list2[n];
        }else{
            total_difference += list2[n] - list1[n];
        }

        map1.entry(list1[n])
        .and_modify(|count| *count += 1)
        .or_insert(1); 

        map2.entry(list2[n])
        .and_modify(|count| *count += 1) 
        .or_insert(1);

    }


    let mut similarity_score: i32 = 0;

    for n in map1.iter(){
        if let Some(val) = map2.get(n.0){
            similarity_score += val*n.0*n.1; 
        }

    }

    println!("Total Difference: {}", total_difference);
    println!("Similarity Score: {}", similarity_score);
    Ok(())
}
