use crate::file_sort::FileSort;

use std::fs::File;
use std::io::{BufReader, Write};

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
            descending: descending
        }
    }
}

impl FileSort for WordSort {
    fn read(&mut self) {
        let file = File::open(&self.input_file).expect("input file doesn't exist");
        let buf = BufReader::new(file);
    }

    fn write(&mut self) {
        println!("{0}", self.output_file);

    }

    fn sort(&mut self) {
        if !self.descending {
            self.lines.sort();
        } else {
            self.lines.sort_by(|a, b| b.cmp(a));  
        }  
    }
}