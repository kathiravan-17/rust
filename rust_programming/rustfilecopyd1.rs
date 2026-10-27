
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn copy_file(src: &str, dst: &str) -> io::Result<u64> {
    let mut source = File::open(src)?;
    let mut destination = File::create(dst)?;

    let mut buffer = [0u8; 4096];
    let mut total_bytes = 0u64;

    loop {
        let bytes_read = source.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        destination.write_all(&buffer[..bytes_read])?;
        total_bytes += bytes_read as u64;
    }

    destination.flush()?;
    Ok(total_bytes)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: filecopy <source> <destination>");
        std::process::exit(1);
    }

    let bytes = copy_file(&args[1], &args[2])?;

    println!("Copied {} bytes successfully", bytes);
    Ok(())
}
