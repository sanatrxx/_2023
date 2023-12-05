use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut digit1 = 0u32;
    let mut digit2 = 0u32;
    let mut sum = 0u32;

    let i = 0;

    let lines = read_lines("input/day1.txt").unwrap().map(|line| line.unwrap());
   
    for line in lines {
        let v = line.as_bytes();

        for &x in v {
           if x >= 48 && x <= 57 {
                digit1 = x as u32 - 48;
                break;
           }
        }

        for &x in v.iter().rev() {
            if x >= 48 && x <= 57 {
                digit2 = x as u32 - 48;
                break;
            }
        }

        sum += digit1*10 + digit2;
    }

    print!("sum: {}", sum);
}




fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}