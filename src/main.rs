fn main() {
    const c: char = 27 as char;
    for i in 0..1000000 {
        print!("{}[2J", c);
        println!("{}", i);
    }
}
