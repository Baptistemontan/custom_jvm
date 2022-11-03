use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    mem::size_of,
    time::Instant,
};

use parser::classfile::classfile::parse_class_file;

pub mod parser;
mod runtime;

struct Tmp<I> {
    iter: I,
}

impl<I, T, E> Iterator for Tmp<I>
where
    I: Iterator<Item = Result<T, E>>,
    E: Error,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.iter.next() {
            Some(value.unwrap())
        } else {
            None
        }
    }
}

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
