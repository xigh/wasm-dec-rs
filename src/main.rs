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

fn leu32_at(v: &Vec<u8>, index:usize) -> Option<u32> {
    let sz = mem::size_of::<u32>();
    let slice = v.get(index..index+sz)?;
    let y = slice.try_into();
    Some(u32::from_le_bytes(y.unwrap()))
}

fn leb128_at(v: &Vec<u8>, index:usize) -> (u64, usize) {
    let mut res = 0u64;
    let mut shift = 0;
    let mut idx = index;
    let len = v.len();
    loop {
        if index >= len {
            return (0, 0);
        }
        let byte = v[idx];
        idx += 1;
        let word = byte as u64;
        // println!("{:x} shift {}", word, shift);
        res |= (word & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return (res, idx - index);
        }
        shift += 7;
    }
}

fn utf8_at(_v: &Vec<u8>, _index:usize) -> (String, usize) {
    ("todo".to_string(), 0)
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

    let magic = leu32_at(&buffer, 0).unwrap();
    if magic != 0x6d736100 {
        return Err("wasm magic not found".to_string());
    }
    println!("Wasm magic {:x}", magic);

    let version = leu32_at(&buffer, 4).unwrap();
    if version != 1 {
        return Err("unsuppored wasm version".to_string());
    }
    println!("\tversion {:x}", version);

    let mut pos = 8usize;
    loop {
        // we need at least 1 bytes : type + size
        if pos >= len {
            break
        }
        let byte = buffer[pos];
        let (size, tmp_size) = leb128_at(&buffer, pos+1);
        if tmp_size == 0 {
            return Err("invalid number of bytes".to_string());
        }

        println!();
        match byte {
            0 => {
                // custom
                println!("section \"custom\"");
                println!("\t{}", utf8_at(&buffer, pos+tmp_size).0);
            },
            1 => {
                // type
                println!("section \"type\"");
            },
            2 => {
                // import
                println!("section \"import\"");
            },
            3 => {
                // function
                println!("section \"function\"");
            },
            4 => {
                // table
                println!("section \"table\"");
            },
            5 => {
                // memory
                println!("section \"memory\"");
            },
            6 => {
                // global
                println!("section \"global\"");
            },
            7 => {
                // export
                println!("section \"export\"");
            },
            8 => {
                // start
                println!("section \"start\"");
            },
            9 => {
                // element
                println!("section \"element\"");
            },
            0x0a => {
                // code
                println!("section \"code\"");
            },
            0x0b => {
                // section "data"
                println!("section \"data\"");
            }
            _ => return Err(format!("unexpected byte {:x}", byte)),
        }
        // println!("\t{:x} bytes [tmp_size={}]", size, tmp_size);
        println!("\t{:x} bytes", size);
        pos += (size as usize) + 1 + tmp_size;
    }

    // println!("{}: {} bytes", name, len);
    // for (index, value) in buffer.iter().enumerate() {
    //     println!("{}: {:x}", index, value);
    // }

    Ok(())
}
