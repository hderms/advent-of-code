#![feature(test)]

extern crate test;

use std::{fs, io::Result};
use std::{fs::File, io::Read};
use test::Bencher;

fn main() {
    let input1 = load_file(String::from("input.txt")).unwrap();
    for _i in 1..10000 {
        let result = process_file(&input1);
        assert_eq!(result, 10584);
    }
}
fn load_file(file_name: String) -> Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut buf: Vec<u8> =Vec::with_capacity(file.metadata().unwrap().len() as usize);
    let _ = file.read_to_end(&mut buf)?;
    Ok(buf)
}

#[inline(always)]
fn process_file(file: &Vec<u8>) -> u16 {
    let mut vec: Vec<u8> = Vec::with_capacity(file.len());
    for d in file {
        let c = *d;
        match vec.pop() {
            Some(previous) => {
                if previous ^ c != 32 {
                    vec.push(previous);
                    vec.push(c);
                }
            }
            None => vec.push(c),
        }
    }
    vec.len() as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        let input1 = load_file(String::from("input.txt")).unwrap();
        b.iter(|| process_file(&mut input1.clone()));
    }
}
