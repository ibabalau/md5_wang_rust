use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

fn find_iv(file: File) -> Result<String, io::Error>
{
    let buf_reader = BufReader::new(file);
    let mut buf = vec![0u8; 64];
    buf_reader.read_exact(&mut buf)?;
    println!("{:?}", buf);
}

fn main() {
    println!("MD5 Collision in Rust Program Started");
    let args: Vec<String> = env::args().collect();

    let option = &args[1];
    let filename = &args[2];
    let file = File::open(filename)?;
    find_iv(file);
}