use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;

use clap::command;
use clap::Parser;
use count_unique_cvm::CountUnique;
use rand::prelude::*;
use rand::rngs::StdRng;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, default_value_t = 1)]
    repeat: usize,
}

fn estimate(reader: &mut BufReader<File>) {
    let mut ctr = CountUnique::new(StdRng::from_entropy(), 1024);
    let mut buf: Vec<u8> = vec![];
    for t in reader.split(b' ') {
        if t.is_ok() {
            ctr.add_token(String::from_utf8(t.unwrap()).unwrap());
        } else {
            break;
        }
        buf.clear();
    }
    println!("{}", ctr.estimate().unwrap());
}

fn main() {
    let args = Args::parse();
    let input = File::open(args.filename).unwrap();
    let mut reader = BufReader::new(input);
    for _ in 0..args.repeat {
        estimate(&mut reader);
        reader.seek(std::io::SeekFrom::Start(0)).unwrap();
    }
}
