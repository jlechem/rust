use crate::file_sort::FileSort;

use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};

pub struct LineSort {
    lines: Vec<String>,
    input_file: String,
    output_file: String,
    descending: bool
}

impl LineSort {
    pub fn new( input:String, output:String, descending:bool ) -> LineSort {
        LineSort {
            lines: Vec::new(),
            input_file: input,
            output_file: output,
            descending
        }
    }
}

impl FileSort for LineSort {
    fn read(&mut self) -> Result<bool,Error> {
        let file = File::open(&self.input_file)?;

        let buffer: BufReader<File> = BufReader::new(file);
        
        for line in buffer.lines() {
            let new_line:String = line.unwrap_or_default();

            if !new_line.is_empty() {
                self.lines.push(new_line);
            }
        }

        return Ok(true);
    }

    fn write(&mut self) -> Result<bool,Error>{
       let mut file: File = File::create(&self.output_file)?;

        for line in &self.lines {
            write!(file,"{}\n", line)?;
        }

        return Ok(true);
    }

    fn sort(&mut self) {
        if !self.descending {
            self.lines.sort();
        } else {
            self.lines.sort_by(|a, b| b.cmp(a));  
        }
    }
}