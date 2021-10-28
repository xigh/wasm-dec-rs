use std::env;

fn main() {
    for (index, argument) in env::args().enumerate() {
        if index > 0 {
            println!("{}", argument);
        }
    }
}
