use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

use parser::classfile::classfile::parse_class_file;

mod parser;
mod runtime;
mod runtime_types;

fn main() {
    let start = Instant::now();

    // let file = File::open("sample/Object.class").unwrap();
    // let file = File::open("sample/System.class").unwrap();
    let file = File::open("sample/HelloWorld.class").unwrap();

    let buff_reader = BufReader::new(file);

    let mut bytes = buff_reader.bytes();

    let class_file = parse_class_file(&mut bytes).unwrap();

    let elapsed = start.elapsed();

    println!("{:#?}", class_file);

    println!("{:?}", elapsed);
}
