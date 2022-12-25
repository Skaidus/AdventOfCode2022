
use std::str::Split;
use std::fs::File;
use std::io::{self, BufRead};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
enum Packet {
    List(Vec<Packet>),
    Integer(u32)
}




impl Packet {
    fn le(&self, other: &Self) -> bool {
        return match (self, other){
            (Self::Integer(a), Self::Integer(b))=> a<=b,
            (Self::List(a), Self::List(b))=>{
                for i in 0..a.len().min(b.len()) {
                    if !a[i].le(&b[i]){
                        return false
                    }
                }
                a.len() <= b.len()
            },
            (Self::List(_), Self::Integer(b))=>
                self.le(&Packet::List(vec![Packet::Integer(*b)])),
            (Self::Integer(_), Self::List(_))=> self.le(other)
        }
    }
}


fn get_packet(tokens: &mut Split<&str>) -> Packet{
    let mut packet : Vec<Packet> = vec![];
    loop {
        match tokens.next() {
            Some(t) => {
                match t {
                    "]" => break,
                    "[" => packet.push(get_packet(tokens)),
                    "" => {},
                    _ => packet.push(Packet::Integer(t.parse::<u32>().unwrap()))
                }
            },
            None => break
        }
    }
    return Packet::List(packet)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = io::BufReader::new(file).lines().into_iter();
    
    let mut index = 1;
    let mut sum = 0;
    loop {
        match reader.next() {
            Some(Ok(ip)) => {
                let ip = ip.replace("[", "[,").replace("]", ",]");
                let packet1 = get_packet(&mut ip.split(","));
                let packet2 = get_packet(&mut reader.next().unwrap().unwrap().replace("[", "[,").replace("]", ",]").split(","));
                if packet1.le(&packet2) {sum+=index}
                index+=1;
                reader.next();
            },
            _ => break
        }
    }
    
}