use std::env;

fn main() {
    for (index, argument) in env::args().enumerate() {
        if index > 0 {
            let rval = process(&argument);
            match rval {
                Err(err) => {
                    eprintln!("process \"{}\" failed with: {}", argument, err)
                }
                _ => {}
            }
        }
    }
}

fn process(_name: &String) -> std::result::Result<bool, &'static str> {
    return Err("not implemented")
}
