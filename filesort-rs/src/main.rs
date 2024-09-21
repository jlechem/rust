mod args;
mod line_sort;
mod file_sort;
mod word_sort;

use clap::Parser;
use file_sort::FileSort;
use line_sort::LineSort;
use word_sort::WordSort;

fn main() {
    let args = args::Args::parse();

    if (!args.word && !args.line) ||
        args.line {
            let mut linesort: LineSort = LineSort::new(args.input_file, args.output_file, args.descending);

            linesort.read();
            linesort.sort();
            linesort.write();
    }
    else {
        let mut wordsort: WordSort = WordSort::new(args.input_file, args.output_file, args.descending);

        wordsort.read();
        wordsort.sort();
        wordsort.write();
    }
}