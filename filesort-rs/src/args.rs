use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub is_word: bool,

    #[arg(short, long)]
    pub is_line: bool
}