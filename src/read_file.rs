use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};

/// Get an iterator that visits a file line-by-line.
pub fn get_lines(path: &str) -> Lines<BufReader<File>> {
    let buf = get_bufreader(path);
    buf.lines()
}

/// Get a simple bufreader
pub fn get_bufreader(path: &str) -> BufReader<File> {
    let f = File::open(path).expect("Error opening file.");
    let buf = BufReader::new(f);
    buf
}

pub fn read_to_string(path: &str) -> Result<String, &'static str> {
    let mut bufreader = get_bufreader(path);
    let mut buf = String::new();
    match bufreader.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err("UTF error in file."),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
