use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
// use std::vec;

fn main() {
    for (index, argument) in env::args().enumerate() {
        if index > 0 {
            if let Err(err) = process(&argument) {
                eprintln!("process \"{}\" failed with: {}", argument, err)
            }
        }
    }
}

fn process(name: &String) -> std::result::Result<(), String> {
    let f = File::open(name);
    if let Err(err) = f {
        return Err(err.to_string());
    }

    let mut reader = BufReader::new(f.unwrap());
    let mut buffer = Vec::new();

    if let Err(err) = reader.read_to_end(&mut buffer) {
        return Err(err.to_string());
    }

    println!("{}: {} bytes", name, buffer.len());
    for (index, value) in buffer.iter().enumerate() {
        println!("{}: {:x}", index, value);
    }

    Ok(())
}
