use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

use parser::classfile::classfile::parse_class_file;

pub mod parser;
mod runtime;

fn main() {
    let start = Instant::now();

    let file = File::open("sample/HelloWorld.class").unwrap();

    let buff_reader = BufReader::new(file);

    let mut bytes = buff_reader.bytes();

    let class_file = parse_class_file(&mut bytes).unwrap();

    let elapsed = start.elapsed();

    dbg!(class_file);

    println!("{:?}", elapsed);
}
