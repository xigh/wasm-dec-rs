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

fn leb128_at(v: &Vec<u8>, index:usize) -> (usize, usize) {
    let mut res = 0usize;
    let mut shift = 0;
    let mut idx = index;
    let len = v.len();
    loop {
        if index >= len {
            return (0, 0);
        }
        let byte = v[idx];
        idx += 1;
        let word = byte as usize;
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
        let (section_size, word_size) = leb128_at(&buffer, pos+1);
        if word_size == 0 {
            return Err("invalid number of bytes".to_string());
        }

        println!();
        match byte {
            0x00 => {
                // custom
                println!("section \"custom\"");
                println!("\t{}", utf8_at(&buffer, pos+word_size).0);
            },
            0x01 => {
                // type
                println!("section \"type\"");
                let mut start = pos+1+word_size;
                let end = start+section_size;
                dump_bytes(&buffer, start, end);
                let tcount = buffer[start];
                start += 1;
                for n in 0..tcount {
                    let ftype = buffer[start];
                    start += 1;
                    if ftype != 0x60 {
                        panic!("expected function-type, got {:02x}", ftype);
                    }
                    println!("\tfunc {}/{}:", n, tcount);

                    let pcount = buffer[start];
                    start += 1;
                    for p in 0..pcount {
                        let ptype = buffer[start];
                        start += 1;
                        let tname = match ptype {
                            0x7f => "i32",
                            0x7e => "i64",
                            0x7d => "f32",
                            0x7c => "f64",
                            0x70 => "funcref",
                            0x6f => "externref",
                            0x60 => "func",
                            0x40 => "resulttype",
                            _ => "unkown",
                        };
                        println!("\t\tparam {}: type={:02x} {}", p, ptype, tname);
                    }

                    let rcount = buffer[start];
                    start += 1;
                    for r in 0..rcount {
                        let rtype = buffer[start];
                        start += 1;
                        let tname = match rtype {
                            0x7f => "i32",
                            0x7e => "i64",
                            0x7d => "f32",
                            0x7c => "f64",
                            0x70 => "funcref",
                            0x6f => "externref",
                            0x60 => "func",
                            0x40 => "resulttype",
                            _ => "unkown",
                        };
                        println!("\t\tresult {}: type={:02x} {}", r, rtype, tname);
                    }
                }
            },
            0x02 => {
                // import
                println!("section \"import\"");
            },
            0x03 => {
                // function
                println!("section \"function\"");
                let start = pos+1+word_size;
                let end = start+section_size;
                dump_bytes(&buffer, start, end);
            },
            0x04 => {
                // table
                println!("section \"table\"");
            },
            0x05 => {
                // memory
                println!("section \"memory\"");
                let start = pos+1+word_size;
                let end = start+section_size;
                dump_bytes(&buffer, start, end);
            },
            0x06 => {
                // global
                println!("section \"global\"");
            },
            0x07 => {
                // export
                println!("section \"export\"");
                let start = pos+1+word_size;
                let end = start+section_size;
                dump_bytes(&buffer, start, end);
            },
            0x08 => {
                // start
                println!("section \"start\"");
            },
            0x09 => {
                // element
                println!("section \"element\"");
            },
            0x0a => {
                // code
                println!("section \"code\"");
                let start = pos+1+word_size;
                let end = start+section_size;
                dump_bytes(&buffer, start, end);
            },
            0x0b => {
                // section "data"
                println!("section \"data\"");
            }
            _ => return Err(format!("unexpected byte {:x}", byte)),
        }
        // println!("\t{:x} bytes [tmp_size={}]", size, tmp_size);
        println!("\t{:x} bytes", section_size);
        pos += (section_size as usize) + 1 + word_size;
    }

    // println!("{}: {} bytes", name, len);
    // for (index, value) in buffer.iter().enumerate() {
    //     println!("{}: {:x}", index, value);
    // }

    Ok(())
}

fn dump_bytes(buffer:&Vec<u8>, start:usize, end:usize) {
    println!("\trange=[{}-{}]", start, end-1);
    print!("\t");
    for i in start..end {
        print!("{:02x} ", buffer[i])
    }
    println!()
}