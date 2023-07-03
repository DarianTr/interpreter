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
        Ok(input) => {
            println!("{:?}", input);
            let result = runtime::run(input.0, input.1);
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
