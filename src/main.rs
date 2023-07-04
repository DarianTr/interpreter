use clap::Parser;
mod parser;
mod runtime;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}
fn main() {
    let args = Args::parse();
    let x = parser::parser(args.file);
    match x {
        Ok(mut input) => {
            let result = runtime::run(&mut input);
            match result {
                Ok(_) => {}
                Err(e) => println!(
                    "An error occured while running in Line {:?}: {:?}",
                    e.line, e.msg
                ),
            }
        }
        Err(e) => println!("Error in Line {:?}: {:?}", e.line, e.msg),
    }
}
