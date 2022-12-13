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

fn main() {
    println!("Hello, world!");
}