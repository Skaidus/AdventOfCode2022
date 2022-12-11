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
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let a : Vec<&str> = ip.split_whitespace().collect();
                let enemy = a[0].chars().nth(0).unwrap() as u32 - 64u32;
                let you = a[1].chars().nth(0).unwrap() as u32 - 87u32;
                sum += you + match (enemy, you){
                    (2,3) | (3,1) | (1,2) => 6,
                    (a,b) => {
                        if a==b {3} else {0}
                    }
                };
            }
        }
        println!("total score: {}", sum);
    }
    
}
