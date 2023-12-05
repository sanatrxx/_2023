use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const R: usize = 140;
const C: usize = 140;

fn main() {

    let mut x1 = 0;
    let mut x2 = 0;
    let mut mx = [[0u8; C]; R];
    let mut n = 0;
    let mut sum = 0;

    let mut parse = false;

    let lines = read_lines("input/day3.txt").unwrap().map(|line| line.unwrap());

    let mut i = 0;
    for line in lines {

        let mut j = 0;
        for &c in line.as_bytes() {
            mx[i][j] = c;
            j += 1;
        }
         
        i += 1;
    }

    i = 0;
    let mut exp = 0;
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

                if is_valid(mx, x1, x2, i) {
                    sum += n;
                }   
                parse = false;
            }
        }

        if parse {
            if is_valid(mx, x1, C-1, i) {
                sum += n;
            }  
            parse = false;
        }
        
    }

    print!("sum: {sum}");
}

fn is_valid (mx: [[u8; C]; R], x1: usize, x2: usize, y:usize) -> bool {
    let startx = if x1 != 0 {x1 - 1}
                    else {x1};
    let starty = if y != 0 {y - 1}
                    else {y};

    let mut l = x2-x1+1;
    if x1 != 0 {l += 1;}
    if x2 != C-1 {l += 1;}

    let mut h = 1;
    if y != 0 {h += 1;}
    if y != R-1 {h += 1;}

    let mut f = false;
    for i in starty .. starty+h {
        for j in startx .. startx+l {
            if mx[i][j] != 46 && !(mx[i][j] >= 48 && mx[i][j] <= 57) {
                f = true;
                break;
            }
        }
        if f {break;}
    }

    f
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}