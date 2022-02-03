use std::env;

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    println!("{} arguments : {:?}", arguments.len(), arguments);
}

