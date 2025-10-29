use std::fs;
use std::io;

fn main() {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => match fs::read(buf.trim()) {
            Ok(_) => println!("success"),
            _ => println!("failure"),
        },
        _ => println!("failure"),
    }
}
