use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUMS:  [&'static str; 10] = ["zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"];

fn main() {
    let mut digit1 = 0;
    let mut digit2 = 0;
    let mut index = 0;
    let mut min_index = 0;
    let mut max_index = 0;
    let mut sum = 0u32;

    let lines = read_lines("input/day1.txt").unwrap().map(|line| line.unwrap());
    let mut z = 0;
    for line in lines {
        let v = line.as_bytes();

        println!("rrr {}", v.len());
    
        for (i, &x) in v.iter().enumerate() {
            if is_digit(x) {
                digit1 = x as u32 - 48;
                min_index = i;
                break;
            }
        }

        index = min_index;
        for i in 0 .. 10 {
            
            match subs(v, NUMS[i].as_bytes(), &mut index) {
                Some(x) => {
                    if index < min_index {
                        min_index = index;
                        digit1 = i as u32;
                    }
                }
                None => {}
            }
        }

        for (i, &x) in v.iter().rev().enumerate() {
            if is_digit(x) {
                digit2 = x as u32 - 48;
                max_index = v.len() - i - 1;
                break;
            }
        }

        index = max_index;
        for i in 0 .. 10 {

            
            match subs_rev(v, NUMS[i].as_bytes(), &mut index) {
                Some(x) => {
                    if index > max_index {
                        max_index = index;
                        digit2 = i as u32;
                    }
                }
                None => {}
            }
        }

        z += 1;
        sum += digit1*10 + digit2;
        print!("{0} {1}\n", z, digit1*10 + digit2);
        
    }
    
    print!("sum: {}", sum);
}

fn is_digit (c: u8) -> bool {
    if c >= 48 && c <= 57 {true}
    else {false}
}

fn subs (s: &[u8], subs: &[u8], index: &mut usize) -> Option<usize> {

    for i in 0 .. *index {
        if s[i] == subs[0] {
            
            let mut f = true;
            for j in 1 .. subs.len() {
                if i+j < s.len() {
                    if (s[i+j] != subs[j]) {
                        f = false;
                    }
                }
                
            }

            if f  {
                *index = i;
                return Some(i);
            }
        }
    }
    return None;  
}

fn subs_rev (s: &[u8], subs: &[u8], index: &mut usize) -> Option<usize> {

    let mut i = s.len()-1;

    loop {
        if s[i] == subs[0] {

            let mut f = 1;
            for j in 1 .. subs.len() {
                if i+j < s.len() {
                    if (s[i+j] != subs[j]) {
                        f = 0;
                        break;
                    }
                }
                else {
                    f = 0;
                    break;
                }
                
            }

            if f == 1 {
                *index = i;
                return Some(i);
            }

        }

        if i <= *index { break;}
        i -= 1;
    }

    

    return None;  
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}