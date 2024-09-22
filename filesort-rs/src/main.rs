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

            println!("Filesort start\n");
            println!("Reading file data (by Line)\n");

            let result = linesort.read();

            match result {
                Ok(_res) => { 
                },
                Err(e) => { 
                    eprintln!("An error occured with the input file\n{}", e);
                    return;
                }
            }
            
            println!("Soritng data\n");
            
            linesort.sort();

            println!("Writing file data\n");

            let result = linesort.write();

            match result {
                Ok(_res) => { 
                },
                Err(e) => { 
                    eprintln!("An error occured with the output file\n{}", e);
                    return;
                }
            }

            println!("Filesort complete\n");
    }
    else {
        let mut wordsort: WordSort = WordSort::new(args.input_file, args.output_file, args.descending);

        println!("Filesort start\n");
        println!("Reading file data (by Word)\n");

        let result = wordsort.read();

        match result {
            Ok(_res) => { 
            },
            Err(e) => { 
                eprintln!("An error occured with the input file\n{}", e);
                return;
            }
        }

        println!("Soritng data\n");
        
        wordsort.sort();
        
        println!("Writing file data\n");

        let result = wordsort.write();

        match result {
            Ok(_res) => { 
            },
            Err(e) => { 
                eprintln!("An error occured with the output file\n{}", e);
                return;
            }
        }

        println!("Filesort complete\n");

    }
}