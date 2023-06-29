use clap::Parser;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}
fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let x = parser::parser(args.file);
    println!("{:?}", x);
}
