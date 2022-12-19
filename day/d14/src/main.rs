use std::i32::MIN;
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

fn get_units(mut grid : Vec<Vec<bool>>, column_offset : i32) -> u32 {
    let mut units : u32 = 0;
    let mut pos = (0,500-column_offset);
    let rows : i32 = grid.len() as i32; 
    let cols : i32 = grid[0].len() as i32;
    let directions = [(1,0), (1,-1), (1,1)];
    let mut out_limits = false;
    // REFORMAR: AHORA MISMO SE CHECKEA SI BLOCKED OUT OF LIMITS
    loop {
        units+=1;
        while (0<= pos.0) && (pos.0 < rows) && (0<= pos.1) && (pos.1 < cols) {
            for dir in directions{
                let candidate = (pos.0 + dir.0, pos.1 + dir.1);
                println!("({} {})", candidate.0, candidate.1);
                out_limits = !((0<= candidate.0) && (candidate.0 < rows) && (0<= candidate.1) && (candidate.1 < cols));
                if out_limits || grid[candidate.0 as usize][candidate.1 as usize] {
                    pos = candidate;
                    break
                }
            }
            if !out_limits {grid[pos.0 as usize][pos.1 as usize] = true;}

        }
        return units
    }
}

fn main() {
    
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let (mut max_i, mut min_j, mut max_j) = (MIN, 500, 500);
        for line in lines {
            if let Ok(ip) = line {  
                for x in ip.split(" -> "){
                    let mut y = x.split(',')
                    .map(|x| x.parse::<i32>().expect("parse error"));
                    let i = y.next().unwrap();
                    max_i = max_i.max(i);
                    let j = y.next().unwrap();
                    min_j = min_j.min(j);
                    max_j = max_j.max(j);
                }
            }
        }
        
        let mut grid = vec![vec![false; (max_j + 1 - min_j) as usize]; (max_i + 1) as usize];
        if let Ok(lines) = read_lines("input.txt") {
        
        for line in lines {
            if let Ok(ip) = line {
                let mut corners = ip.split(" -> ")
                .map(|x| x.split(',').map(|x| x.parse::<i32>().unwrap())).map(|mut x| (x.next().unwrap() as usize, (x.next().unwrap()-min_j) as usize)).into_iter();
                let mut begin  = corners.next().unwrap();
                for corner in corners {
                    if corner.0 == begin.0 {
                        for x1 in begin.1..corner.1 {
                            grid[corner.0][x1] = true;
                        }
                    } else {
                        for x0 in begin.0..corner.0 {
                            grid[x0][corner.1] = true;
                        }
                    }
                    begin = corner;
                }
            }
        }
        
        println!("Blocks: {}", get_units(grid, min_j));
    }

        // let next_cycle = ((line_count-20)/40 + 1)*40 + 20;
        // for c in (next_cycle..=220).step_by(40){
        //     sum += c * value;
        // }
        

    }

 
        

        // let next_cycle = ((line_count-20)/40 + 1)*40 + 20;
        // for c in (next_cycle..=220).step_by(40){

        
}