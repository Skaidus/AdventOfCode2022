use std::i32::{MAX,MIN};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_units(mut grid : Vec<Vec<bool>>) -> u32 {
    let mut units : u32 = 0;
    let mut pos = (0,0);
    let rows : i32 = grid.len() as i32; 
    let cols : i32 = grid[0].len() as i32;
    let directions = [(1,0), (1,-1), (1,1)];
    loop {
        units+=1;
        while (0<= pos.0) && (pos.0 < rows) && (0<= pos.1) && (pos.1 < cols) {
            for dir in directions{
                let candidate = (pos.0 + dir.0, pos.1 + dir.1);
                if grid[candidate.0 as usize][candidate.1 as usize] {
                    pos = candidate;
                    break
                }
            }
            grid[pos.0 as usize][pos.1 as usize] = true;
            break
        }
        return units
    }
}

fn main() {
    
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let (mut sum, mut value, mut line_count) = (0,1,0);
        let (mut min_i, mut max_i, mut min_j, mut max_j) = (MAX, MIN, MAX, MIN);
        for line in lines {
            if let Ok(ip) = line {  
                ip.split(" -> ").for_each(move |x|{
                    let mut y = x.split(',')
                    .map(|x| x.parse::<i32>().expect("parse error"));
                    let i = y.next().unwrap();
                    min_i = min_i.min(i);
                    max_i = max_i.max(i);
                    let j = y.next().unwrap();
                    min_j = min_j.min(j);
                    max_j = max_j.max(j);
                });
            }
        }
        

        // let next_cycle = ((line_count-20)/40 + 1)*40 + 20;
        // for c in (next_cycle..=220).step_by(40){
        //     sum += c * value;
        // }
        println!("total size: {}", sum);

    }
        
}