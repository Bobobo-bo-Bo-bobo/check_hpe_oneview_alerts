use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn read_password_from_file(f: &str) -> Result<String, Box<dyn Error>> {
    let mut line = String::new();
    let fd = fs::File::open(f)?;
    let mut reader = BufReader::new(fd);
    reader.read_line(&mut line)?;

    if line.ends_with('\n') {
        line.pop();
    }
    if line.ends_with('\r') {
        line.pop();
    }

    if line.is_empty() {
        bail!("password file is empty")
    }

    Ok(line)
}

pub fn read_file(f: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let result = fs::read(f)?;
    Ok(result)
}
