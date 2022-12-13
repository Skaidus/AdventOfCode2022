enum Packet {
    List(Vec<Packet>),
    Integer(u32)
}

fn main() {
    println!("Hello, world!");
}