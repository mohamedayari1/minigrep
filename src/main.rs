use std::env;

fn main() {
    println!("Hello distributed systems engineer!");
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
