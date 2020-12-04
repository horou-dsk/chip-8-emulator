use std::fs::File;
use std::io::Read;

fn main() {
    let file = File::open("roms/PONG");
    match file {
        Ok(mut file) => {
            let mut v = Vec::new();
            file.read_to_end(&mut v);
            println!("{:?}", v);
        }
        Err(_) => {}
    }
}