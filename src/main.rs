use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut w = HashSet::new();
    let mut g = HashSet::new();
    let mut m: Vec<usize> = Vec::new();
    let mut cnt;

    let lines = read_lines("input/bigboy.txt").unwrap().map(|line| line.unwrap());
    
    for s in lines {
        let x = s.split(": ").collect::<Vec<_>>()[1].split("|").collect::<Vec<_>>();
        w = x[0].split_ascii_whitespace().collect();
        g = x[1].split_ascii_whitespace().collect();

        cnt = 0;
        for x in g {
            if w.contains(x) {
                cnt += 1;
            }
        }

        m.push(cnt);
    }

    cnt = 0;
    for i in 0 .. m.len() {
        solve(&m, &mut cnt, i+1, m[i]);
        cnt += 1;
    }

    print!("{cnt} scratchcards counted!");
}

fn solve (v: &Vec<usize>, cnt: &mut usize, pos: usize, n: usize) {
    
    let dx = if pos+n > v.len() {v.len()}
                else {pos+n};

    for i in pos .. dx {
        solve(v, cnt, i+1, v[i]);
        *cnt += 1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}