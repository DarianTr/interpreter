mod parser;

fn main() {
    let x = parser::parser("test.txt".to_owned());
    println!("{:?}", x);
}
