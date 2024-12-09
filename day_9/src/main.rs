use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn checksum(disk: Vec<i32>) -> i64 {
    let mut check = 0;
    for n in 0..disk.len(){
        if disk[n] != -1 {
            check += n as i64 * disk[n] as i64;
        }
    }
    check
}

fn replace_range(disk: &mut Vec<i32>, start: usize, end: usize, value: i32) {
    for i in start..end {
        if i < disk.len() {
            disk[i] = value;
        }
    }
}

fn print_disk(disk: Vec<i32>){
    for a in disk{
        if a != -1{
            print!("{}",a);
        }else{
            print!(".");
        }
    }
    println!("");
}

fn rearrange(mut disk: Vec<i32>) -> Vec<i32> {
    for n in (0..disk.len()).rev() {
        if disk[n] != -1 {
            let tmp = disk[n];
            disk[n] = -1;
            for i in 0..=n {
                if disk[i] == -1 {
                    disk[i] = tmp;
                    break;
                }
            }
        }
    }
    disk
}

fn rearrange_contiguos(mut disk: Vec<i32>) -> Vec<i32> {
    let mut n_i: i32 = (disk.len()-1).try_into().unwrap();
    while n_i >= 0 {
        let n = n_i as usize;
        if disk[n] != -1 {
            let mut q = 1;
            for a in (0..n).rev(){
                if disk[a] == disk[n] {
                    q+=1;
                }else{
                    break;
                }
            }
            let tmp = disk[n];

            for i in 0..=n {
                if disk[i] == -1 {
                    let mut q2 = 0;
                    for a in i..=n{
                        if disk[a] == -1 {
                            q2+=1;
                        }else{
                            break;
                        }
                    }
                    if q2 >= q {
                        replace_range(&mut disk, n+1-q, n+1, -1);
                        replace_range(&mut disk, i, i+q, tmp);
                        break;
                    }
                }
            }
            n_i -= q as i32;
        }else{
            n_i -= 1
        }
    }
    disk
}

fn main() -> io::Result<()> {
    let file_1 = "file/1.txt";

    let file1 = File::open(file_1)?;
    let reader = BufReader::new(file1);

    let mut disk = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut n = 0;
        let mut space = false;
        for c in line.chars(){
            if !space {
                let c = c.to_digit(10).unwrap();
                for _ in 0..c{
                    disk.push(n);
                } 
                n += 1;
                space = !space;
            }else{
                let c = c.to_digit(10).unwrap();
                for _ in 0..c{
                    disk.push(-1);
                } 
                space = !space;
            }
        }
    }
    let disk1 = rearrange(disk.clone());
    let disk2 = rearrange_contiguos(disk.clone());

    println!("Total checksum: {}", checksum(disk1));
    println!("Total checksum: {}", checksum(disk2));

    Ok(())
}
