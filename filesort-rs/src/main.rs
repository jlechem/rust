mod args;
mod line_sort;
mod file_sort;

use clap::Parser;
use file_sort::FileSort;
use line_sort::LineSort;

fn main() {
    let args = args::Args::parse();

    // If no type of sort is specified we default to Line
    if (!args.word &&  
        !args.line) ||
        args.line {
            let linesort: LineSort = LineSort::new(args.input_file, args.output_file, args.descending);

            linesort.read();
            linesort.sort();
            linesort.write();
    }
    else {

    }
}