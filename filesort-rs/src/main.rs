use clap::Parser;

mod args;

fn main() {
    let args = args::Args::parse();

    println!("Hello {}!", args.is_word);
    println!("Hello {}!", args.is_line);
}