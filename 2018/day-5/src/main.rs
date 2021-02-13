use std::io::Result;
use std::{fs::File, io::Read};
fn main() {
    let mut input1 = load_file(String::from("input.txt")).unwrap();
    for i in 1..10000 {
        let result = process_file(&mut input1.clone());
        assert_eq!(result, 10584);
    }
}
fn load_file(file_name: String) -> Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut buf: Vec<u8> = vec![];
    let _ = file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn process_file(file: &mut Vec<u8>) -> u16 {
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
