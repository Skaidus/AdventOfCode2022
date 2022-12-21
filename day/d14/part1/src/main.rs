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
    let mut pos;
    let xs : i32 = grid.len() as i32; 
    let ys : i32 = grid[0].len() as i32;
    let directions = [(0,1), (-1,1), (1,1)];
    loop {
        pos = (500-column_offset,0);
        'move_block: loop {
            if !((0<= pos.0) && (pos.0 < xs) && (0<= pos.1) && (pos.1 < ys)) {return units}
            for dir in directions{
                let candidate = (pos.0 + dir.0, pos.1 + dir.1);
                if !((0<= candidate.0) && (candidate.0 < xs) && (0<= candidate.1) && (candidate.1 < ys)) 
                || !grid[candidate.0 as usize][candidate.1 as usize] {
                    pos = candidate;
                    continue 'move_block;
                }
            }
            grid[pos.0 as usize][pos.1 as usize] = true;
            units+=1;
            break;
            
        }
    }
}

fn main() {
    
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let (mut max_y, mut min_x, mut max_x) = (0, 500, 500);
        for line in lines {
            if let Ok(ip) = line {  
                for x in ip.split(" -> "){
                    let mut p = x.split(',')
                    .map(|c| c.parse::<i32>().expect("parse error"));
                    let x = p.next().unwrap();
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    let y = p.next().unwrap();
                    max_y = max_y.max(y);

                }
            }
        }
        
        let mut grid = vec![vec![false; (max_y + 1) as usize]; (max_x - min_x + 1) as usize ];
        if let Ok(lines) = read_lines("../input.txt") {
        
        for line in lines {
            if let Ok(ip) = line {
                let mut corners = ip.split(" -> ")
                .map(|p| p.split(',').map(|c| c.parse::<i32>().unwrap())).map(|mut x| ((x.next().unwrap() - min_x) as usize, x.next().unwrap() as usize)).into_iter();
                let mut begin  = corners.next().unwrap();
                for corner in corners {
                    if corner.0 > 10000 {
                        println!("{}",corner.0);
                    }
                    if corner.1 > 10000 {
                        println!("{}",corner.1);
                    }
                    if corner.0 == begin.0 {
                        let min = begin.1.min(corner.1);
                        let max = begin.1.max(corner.1);
                        for y in min..=max {
                            grid[corner.0][y] = true;
                        }
                    } else {
                        let min = begin.0.min(corner.0);
                        let max = begin.0.max(corner.0);
                        for x in min..=max {
                            grid[x][corner.1] = true;
                        }
                    }
                    begin = corner;
                }
            }
        }
        
        println!("Blocks: {}", get_units(grid, min_x));
    }

        // let next_cycle = ((line_count-20)/40 + 1)*40 + 20;
        // for c in (next_cycle..=220).step_by(40){
        //     sum += c * value;
        // }
        

    }

 
        

        // let next_cycle = ((line_count-20)/40 + 1)*40 + 20;
        // for c in (next_cycle..=220).step_by(40){

        
}