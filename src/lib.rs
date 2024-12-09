use std::fmt::Display;
use std::io;
use std::io::prelude::*;

pub fn read_in() -> Result<Vec<String>, io::Error> {
    io::stdin().lock().lines().collect()
}

pub fn write_out<T: Display>(s: T) {
    println!("{}", s);
}
