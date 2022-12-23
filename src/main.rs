use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: saika <file>");
        return Ok(());
    }
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0; 16];
    let mut offset = 0;
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        print!("{:08x}  ", offset);
        for i in 0..bytes_read {
            print!("{:02x} ", buffer[i]);
            if i % 8 == 7 {
                print!(" ");
            }
        }
        for i in bytes_read..16 {
            print!("   ");
            if i % 8 == 7 {
                print!(" ");
            }
        }
        print!(" ");
        for i in 0..bytes_read {
            let c = buffer[i] as char;
            if c.is_ascii_graphic() {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
        offset += bytes_read;
    }
    Ok(())
}

