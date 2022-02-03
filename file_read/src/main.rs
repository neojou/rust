use std::fs;
use std::env;
use std::char;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
	eprintln!("Usage: {} filename", args[0]);
	std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
			.expect("Something went wrong reading the file");

    let mut counts: [i32; 26] = [0; 26];
    for each_c in contents.chars() {
	let c: char  = each_c.to_ascii_lowercase();
	if !c.is_ascii_alphabetic() {
		continue;
	}
	let pos: i32 = (c as i32).wrapping_sub('a' as i32);
	let count = counts[pos as usize];
	counts[pos as usize] = count + 1;
    }

    for i in 0..counts.len() {
	if counts[i] == 0 {
	    let ch: char = (i as u8).wrapping_add('a' as u8) as char;
	    println!("No char: {}", ch);
	}
    }
}

