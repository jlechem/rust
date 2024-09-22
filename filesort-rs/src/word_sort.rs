use crate::file_sort::FileSort;

use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};

pub struct WordSort {
    lines: Vec<String>,
    input_file: String,
    output_file: String,
    descending: bool
}

impl WordSort {
    pub fn new( input:String, output:String, descending:bool ) -> WordSort {
        WordSort {
            lines: Vec::new(),
            input_file: input,
            output_file: output,
            descending
        }
    }
}

impl FileSort for WordSort {
    fn read(&mut self) -> Result<bool,Error> {
        let file = File::open(&self.input_file)?;

        let buffer: BufReader<File> = BufReader::new(file);
        
        for line in buffer.lines() {
            self.lines.push(line.unwrap_or_default());
        }

        return Ok(true);
    }

    fn write(&mut self) {
        let mut file: File = File::create(&self.output_file).expect("Unable to create output file {0}");

        for line in &self.lines {
            write!(file,"{}\n", line).expect("Unable to write data to file.")
        }
    }

    fn sort(&mut self) {
        if !self.descending {
            self.lines.sort();
        } else {
            self.lines.sort_by(|a, b| b.cmp(a));  
        }  
    }
}