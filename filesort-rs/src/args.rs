use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0.0", about = "Utility to sort files by word or line in ascending/descending order", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub word: bool,

    #[arg(short, long)]
    pub line: bool,

    #[arg(short, long)]
    pub descending: bool,

    #[arg(short, long)]
    pub input_file: String,

    #[arg(short, long)]
    pub output_file: String,

}