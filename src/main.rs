#![feature(slice_patterns)]

use std::io;
use std::io::Read;
use std::fs::{self, OpenOptions};

mod engine;
use engine::BrainFuck;
use std::env;

fn main() {
    if let Some(src) = env::args().skip(1).take(1).next() {
        let program = if let Ok(f) = fs::metadata(&src) {
            if f.is_file() {
                let mut s = String::new();
                let mut f = OpenOptions::new().read(true).open(&src).unwrap();
                let _ = f.read_to_string(&mut s);
                s
            } else {
                src
            }
        } else {
            src
        };

        let mut brain = BrainFuck::new(&program);
        let res = brain.eval();
        println!("{}", res.unwrap());
    } else {
        println!("Provide the src file/string as an argument");
    }
}