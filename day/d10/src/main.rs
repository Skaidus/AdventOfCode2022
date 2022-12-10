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
    
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let (mut sum, mut value, mut line_count) = (0,1,0);
        for line in lines {
            if let Ok(ip) = line {
                let step = if ip.as_str() == "noop"{1} else {2};
                for _ in 0..step{
                    line_count+=1;
                    if (line_count-20) % 40 == 0 {
                        sum+= value*line_count;
                    }
                }
                if step == 2 {
                    value += ip.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                }
            }
        
        }
        // let next_cycle = ((line_count-20)/40 + 1)*40 + 20;
        // for c in (next_cycle..=220).step_by(40){
        //     sum += c * value;
        // }
        println!("total size: {}", sum);

    }
        
}

