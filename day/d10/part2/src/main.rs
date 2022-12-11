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

fn main() {
    
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        const ROWS : usize = 6;
        const COLS : usize = 40;
        let mut screen : [[char; COLS];ROWS] = [['.'; COLS];ROWS];
        let (mut value, mut column, mut row) : (i32, i32, usize)= (1,0,0);
        for line in lines {
            if let Ok(ip) = line {
                let step = if ip.as_str() == "noop"{1} else {2};
                for _ in 0..step{
                    if (value-column).abs() <= 1 {screen[row][column as usize] = '#'}
                    column+=1;
                    if column as usize == COLS {
                        column=0; row+=1;
                    }
                }
                if step == 2 {
                    value += ip.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                }
            }
        }
        for i in 0..ROWS{
            for j in 0..COLS{
                print!("{}", screen[i][j]);
            }
            print!("\n");
        }
    }
        
}

