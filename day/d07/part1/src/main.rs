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

struct FileSystem {
    stack: Vec<u32>
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            stack: vec![]
        }
    }

    fn move_in(&mut self)  {
        self.stack.push(0u32)
    }

    fn add_to_end(&mut self, value : u32){
        let t = self.stack.len()-1;
        self.stack[t] += value;
    }

    fn move_out(&mut self)  -> u32{
        match self.stack.pop(){
            Some(value) => {self.add_to_end(value); value},
            None => {0},
        }
    }

    fn add_file(&mut self, file : u32)  {
        if !self.stack.is_empty(){
            self.add_to_end(file)
        }
    }
}


fn main() {
    
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        let mut fs = FileSystem::new();
        for line in lines {
            if let Ok(ip) = line {
                match ip.as_str() {
                    "$ ls" => {},
                    "$ cd .." => {
                        let v = fs.move_out();
                        if v <= 100000{
                            sum += v;
                        }
                    },
                    value =>{
                        if value.starts_with("$ cd") {
                            fs.move_in();
                        } else if !value.starts_with("dir") {
                            let end_bytes = value.find(" ").unwrap_or(value.len());
                            let result = &value[..end_bytes]; 
                            let size = result.parse::<u32>().unwrap();
                            fs.add_file(size);
                        }
                    }

                }
            }
        }
        println!("total size: {}", sum);
    }
}
