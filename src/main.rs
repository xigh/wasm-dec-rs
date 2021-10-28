use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::mem;
// use std::vec;
use core::convert::TryInto;

fn main() {
    for (index, argument) in env::args().enumerate() {
        if index > 0 {
            if let Err(err) = process(&argument) {
                eprintln!("process \"{}\" failed with: {}", argument, err)
            }
        }
    }
}

fn u32_at(v: &Vec<u8>, index:usize) -> Option<u32> {
    let sz = mem::size_of::<u32>();
    let slice = v.get(index..index+sz)?;
    let y = slice.try_into();
    Some(u32::from_le_bytes(y.unwrap()))
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

    let len = buffer.len();
    if len < 8 {
        return Err("file too short".to_string());
    }

    let magic = u32_at(&buffer, 0).unwrap();
    if magic != 0x6d736100 {
        return Err("wasm magic not found".to_string());
    }
    println!("Wasm magic {:x}", magic);

    let version = u32_at(&buffer, 4).unwrap();
    if version != 1 {
        return Err("unsuppored wasm version".to_string());
    }
    println!("\tversion {:x}", version);

    let mut pos = 8usize;
    loop {
        println!();

        // we need at least 2 bytes : type + size
        if pos + 1 >= len {
            break
        }
        let byte = buffer[pos];
        let size = buffer[pos+1];
        match byte {
            1 => {
                // section "type"
                println!("section \"type\"");
                println!("\t{} bytes", size);
                pos += (size as usize) + 2;
            },
            3 => {
                // section "function"
                println!("section \"function\"");
                println!("\t{} bytes", size);
                pos += (size as usize) + 2;
            },
            5 => {
                // section "memory"
                println!("section \"memory\"");
                println!("\t{} bytes", size);
                pos += (size as usize) + 2;
            },
            7 => {
                // section "export"
                println!("section \"export\"");
                println!("\t{} bytes", size);
                pos += (size as usize) + 2;
            },
            10 => {
                // section "code"
                println!("section \"code\"");
                println!("\t{} bytes", size);
                pos += (size as usize) + 2;
            },
            _ => return Err(format!("unexpected byte {:x}", byte)),
        }
    }

    // println!("{}: {} bytes", name, len);
    // for (index, value) in buffer.iter().enumerate() {
    //     println!("{}: {:x}", index, value);
    // }

    Ok(())
}
