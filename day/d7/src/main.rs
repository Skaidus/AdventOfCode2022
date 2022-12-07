
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
    
}
