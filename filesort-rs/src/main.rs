use clap::Parser;

mod args;

fn main() {
    let args = args::Args::parse();

    println!("Hello {}!", args.word);
    println!("Hello {}!", args.line);
}