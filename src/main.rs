use std::{
    fs::File,
    io::Read,
};

use day12::{p1, p2};



fn main() {
    let mut f = File::open("input.txt").expect("can't open file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("can't read file");
    let result = p1(&buf);
    println!("p1: {result}");
    let result = day12::swift::p1(&buf);
    println!("p1_swift: {result}");
    let result = day12::cpp::p1(&buf);
    println!("p1_cpp: {result}");
    let result = p2(&buf);
    println!("p2: {result}");
}

