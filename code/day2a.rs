use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const LIMIT : [u32; 3] = [12, 13, 14];

fn main() {

    let mut max = [0u32; 3];
    let mut pull = [0u32; 3];
    let mut sum = 0;
    let mut count = 1;

    let lines = read_lines("input/day2.txt").unwrap().map(|line| line.unwrap())
        .map(|line| line.replace(&[' ', ','][..], ""))
        .map(|line| line.replace("red", "r"))
        .map(|line| line.replace("green", "g"))
        .map(|line| line.replace("blue", "b"));

    for line in lines {
        
        println!("{line}");

        let x = line.split(';').collect::<Vec<_>>();
        let s = &x[0][5+digits(count)..];

        max = read_pull(s);
        print!{"{:?}\n", max};

        for i in 1 .. x.len() {
            pull = read_pull(x[i]);

            print!{"{:?}\n", pull};
            print!{"max: {:?}\n", max};
            
            for j in 0 .. 3 {
                if pull[j] > max[j] {
                    max[j] = pull[j];
                }
                print!("pull[{j}]: {0} \t max[{j}]: {1}\n", pull[j], max[j]);
            }
        }

        print!{"MAX: {:?}\n", max};

        if max[0] <= LIMIT[0] && max[1] <= LIMIT[1] && max[2] <= LIMIT[2] {
            sum += count;
        }
        count += 1;
    }

    print!("sum: {sum}");
}

fn digits (x:u32) -> usize {
    let mut exp = 1;

    loop {
        if x < 10u32.pow(exp) {break;}
        exp += 1;
    }
    exp as usize
}

fn read_pull (pull: &str) -> [u32;3] {
    let mut aaa = [0,0,0];

    let s = String::from(pull);
    let x = s.split_inclusive(&['r','g','b'][..]);

    for color in x {
        let colorv = color.as_bytes();
        let mut i = colorv.len()-1;
        let mut n = 0u32;
        let mut exp = 0u32;

        let index = match(colorv[i]) {
            114 => {0}, //r
            103 => {1}, //g
            98 => {2}, //b
            _ => {9}
        };

        i -= 1;
        loop {
            n += (colorv[i] as u32 - 48) * (10u32.pow(exp));
            if i == 0 {break;}
            exp += 1; i -= 1;
        }

        aaa[index] = n;
    }
    aaa
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}