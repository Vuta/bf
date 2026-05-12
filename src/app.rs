use std::fs::File;
use std::io::{Read, BufReader};

#[derive(Debug)]
pub struct App {
    pub source: String,
}

impl App {
    pub fn default() -> Self {
        Self { source: String::new() }
    }

    pub fn read_source(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);

        buf_reader.read_to_string(&mut self.source)?;

        Ok(())
    }
}
