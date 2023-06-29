use text_io::*;
use Token::*;
use crate::parser::*;

const ASCII_OFFSET: usize = 97;

pub enum CmpResult {
    Greater,
    Equal,
    Less,
}

pub fn run(input: Vec<Vec<Token>>) {
    let mut virtual_memory = vec![0; 26];
    let mut akku: i32 = 0;
    let mut cmp_result: CmpResult = CmpResult::Equal;

    let mut i = 0;
    loop {
        let line = &input[i];
        match line[0] {
           Ld => {
                match line[1] {
                    Token::Number(x) => {akku = x},
                    Token::Address(x) => {akku = virtual_memory[x as usize - ASCII_OFFSET]},
                    _ => panic!("something went wrong"),
                }
           },
           St => {
                match line[1] {
                    Token::Address(x) => {virtual_memory[x as usize - ASCII_OFFSET] = akku},
                    _ => panic!("something went wrong"),
                }
            },
           In => {
                match line[1] {
                    Token::Address(x) => {
                        let stdin: i32 = read!();
                        virtual_memory[x as usize - ASCII_OFFSET] = stdin;
                    },
                    _ => panic!("something went wrong"),
                }
           },
           Out => {
                match line[1] {
                    Token::Number(x) => {println!("{:?}", x)},
                    Token::Address(x) => {println!("{:?}", virtual_memory[x as usize - ASCII_OFFSET])},
                    _ => panic!("something went wrong"),
                }
           },
           Add => {
                match line[1] {
                    Token::Number(x) => {akku += x},
                    Token::Address(x) => {akku += virtual_memory[x as usize - ASCII_OFFSET]},
                    _ => panic!("something went wrong"),
                }
           },
           Sub => {

                match line[1] {
                    Token::Number(x) => {akku -= x},
                    Token::Address(x) => {akku -= virtual_memory[x as usize - ASCII_OFFSET]},
                    _ => panic!("something went wrong"),
                }
           },
           Mul => {
                match line[1] {
                    Token::Number(x) => {akku *= x},
                    Token::Address(x) => {akku *= virtual_memory[x as usize - ASCII_OFFSET]},
                    _ => panic!("something went wrong"),
                }
           },
           Div => {
                match line[1] {
                    Token::Number(x) => {akku /= x},
                    Token::Address(x) => {akku /= virtual_memory[x as usize - ASCII_OFFSET]},
                    _ => panic!("something went wrong"),
                }
           },
           Mod => {
                match line[1] {
                    Token::Number(x) => {akku %= x},
                    Token::Address(x) => {akku %= virtual_memory[x as usize - ASCII_OFFSET]},
                    _ => panic!("something went wrong"),
                }
           },
           Cmp => {
                match line[1] {
                    Token::Number(x) => {
                        match akku.cmp(&x) {
                           std::cmp::Ordering::Less => {cmp_result = CmpResult::Less},
                           std::cmp::Ordering::Equal => {cmp_result = CmpResult::Equal},
                           std::cmp::Ordering::Greater => {cmp_result = CmpResult::Greater},
                        }
                    },
                    Token::Address(x) => {
                        match akku.cmp(&virtual_memory[x as usize - ASCII_OFFSET]) {
                           std::cmp::Ordering::Less => {cmp_result = CmpResult::Less},
                           std::cmp::Ordering::Equal => {cmp_result = CmpResult::Equal},
                           std::cmp::Ordering::Greater => {cmp_result = CmpResult::Greater},
                        }
                    }
                    _ => panic!("something went wrong"),
                }
           },
           Jmp => {},
           Jlt => {},
           Jeq => {},
           Jgt => {},
           End => {break},
           _ => {panic!("something went wrong")}
        }
        i += 1;
    }
}
