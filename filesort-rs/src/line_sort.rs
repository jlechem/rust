use crate::file_sort::FileSort;

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
            descending: descending
        }
    }
}

impl FileSort for LineSort {
    fn read(&self) {

    }

    fn write(&self) {
        
    }

    fn sort(&self) {
        
    }
}