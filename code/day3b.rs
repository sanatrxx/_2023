use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const R: usize = 140;
const C: usize = 140;

struct Num {n: u32, x: usize, y: usize, l:usize}

fn main() {

    let mut x1 = 0;
    let mut x2 = 0;
    let mut mx = [[0u8; C]; R];
    let mut n = 0;
    let mut sum = 0;


    let mut parse = false;

    let lines = read_lines("input/day3.txt").unwrap().map(|line| line.unwrap());
    let mut v:Vec<Num> = Vec::new();

    let mut i = 0;
    for line in lines {

        let mut j = 0;
        for &c in line.as_bytes() {
            mx[i][j] = c;
            j += 1;
        }
         
        i += 1;
    }

    for i in 0 .. R {
        for j in 0 .. C {

            if mx[i][j] >= 48 && mx[i][j] <= 57 && !parse{
                parse = true;
                n = mx[i][j] as u32 - 48;
                x1 = j;

            }
            else if mx[i][j] >= 48 && mx[i][j] <= 57 && parse {
                n *= 10;
                n += mx[i][j] as u32 - 48;
            }
            else if !(mx[i][j] >= 48 && mx[i][j] <= 57) && parse {
                x2 = j-1;
                v.push(Num { n: n, x: x1, y: i, l: x2-x1+1 });
                parse = false;
            }
        }

        if parse { 
            parse = false;
            v.push(Num { n: n, x: x1, y: i, l: C-x1 });
        }
    }

    for i in 0 .. R {
        for j in 0 .. C {

            let mut cnt = 0;
            let mut gear = 1;
            
            if mx[i][j] == 42 {
                for x in &v {
                    if (x.y >= i-1 && x.y <= i+1) && 
                        ((x.x >= j-1 && x.x <= j+1 ) || ( (x.x + x.l -1) >= j-1 && (x.x + x.l -1) <= j+1 ) ) {
                            gear *= x.n;
                            cnt += 1;
                    }
                }
            }

            if cnt == 2 {
                sum += gear;
            }
        }
    }

    print!("sum: {sum}");
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}