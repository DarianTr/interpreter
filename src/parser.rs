use snafu::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::{self, from_utf8};
use std::vec;

#[derive(Debug, Snafu)]
pub struct ParseError {
    pub msg: String,
    pub line: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Jumpmark {
    pub name: String,
    pub line: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ld,
    St,
    In,
    Out,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Cmp,
    Jmp,
    Jlt,
    Jeq,
    Jgt,
    End,
    Number(i32),
    Address(char),
    Jumpmark(Jumpmark),
    JumpMarkTo(String),
}

fn read_file(file: String) -> Vec<String> {
    let mut vec_of_lines: Vec<String> = vec![];
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(ip) = line {
                vec_of_lines.push(ip);
            }
        }
    }
    return vec_of_lines;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_comment(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut index = bytes.len();
    for idx in 0..bytes.len() {
        if bytes[idx] == 59 {
            //;
            index = idx;
            break;
        }
    }
    let bytes = &bytes[0..index];
    from_utf8(&bytes).unwrap().trim_end().to_owned()
}

pub fn parser(file: String) -> Result<(Vec<Vec<Token>>, Vec<Token>), ParseError> {
    let mut output = vec![];
    let lines = read_file(file);
    let mut line_counter: usize = 0;
    let mut jumpmarks: Vec<Token> = vec![];
    for mut line in lines {
        line = check_comment(&line);
        if line == "" {
            continue;
        }
        let mut words = line.split_whitespace();
        let mut line_output = vec![];
        if let Some(word) = words.next() {
            match word {
                "ld" => line_output.push(Token::Ld),
                "st" => line_output.push(Token::St),
                "in" => line_output.push(Token::In),
                "out" => line_output.push(Token::Out),
                "add" => line_output.push(Token::Add),
                "sub" => line_output.push(Token::Sub),
                "mul" => line_output.push(Token::Mul),
                "div" => line_output.push(Token::Div),
                "mod" => line_output.push(Token::Mod),
                "cmp" => line_output.push(Token::Cmp),
                "jmp" => line_output.push(Token::Jmp),
                "jlt" => line_output.push(Token::Jlt),
                "jeq" => line_output.push(Token::Jeq),
                "jgt" => line_output.push(Token::Jgt),
                "end" => {
                    line_output.push(Token::End);
                    output.push(line_output);
                    continue;
                }
                _ => {
                    return Err(ParseError {
                        msg: format!("Unknown function {}", word).to_string(),
                        line: line_counter,
                    })
                }
            };
        }
        if let Some(word) = words.next() {
            let chars: Vec<char> = word.chars().collect();
            match chars[0] {
                '0'..='9' => {
                    for byte in word.as_bytes() {
                        ensure!(
                            (*byte >= 48 && *byte <= 57),
                            ParseSnafu {
                                msg: "A parameter can only be a letter or a number".to_string(),
                                line: line_counter,
                            }
                        );
                    }
                    line_output.push(Token::Number(word.parse::<i32>().unwrap()));
                }
                '-' => {
                    for byte in &word.as_bytes()[1..word.as_bytes().len()] {
                        ensure!(
                            *byte >= 48 || *byte <= 57,
                            ParseSnafu {
                                msg: "A parameter can only be a letter or a number".to_string(),
                                line: line_counter,
                            }
                        );
                    }
                    line_output.push(Token::Number(word.parse::<i32>().unwrap()));
                }
                'a'..='z' => {
                    match word.as_bytes().len() {
                        2..=usize::MAX => line_output.push(Token::JumpMarkTo(word.to_owned())),
                        1 => line_output.push(Token::Address(word.as_bytes()[0] as char)),
                        _ => {
                            return Err(ParseError {
                                msg: "Something went horribly wrong".to_owned(),
                                line: line_counter,
                            })
                        }
                    };
                }
                ':' => jumpmarks.push(Token::Jumpmark(Jumpmark {
                    name: chars[1..].iter().collect::<String>(),
                    line: line_counter,
                })),
                _ => {
                    return Err(ParseError {
                        msg: "This parameter has to be a letter or a number".to_string(),
                        line: line_counter,
                    })
                }
            };
        } else {
            return Err(ParseError {
                msg: format!("A function needs one parameter"),
                line: line_counter,
            });
        }
        if let Some(j) = words.next() {
            let chars: Vec<char> = j.chars().collect();
            match chars[0] {
                ':' => jumpmarks.push(Token::Jumpmark(Jumpmark {
                    name: chars[1..].iter().collect::<String>(),
                    line: line_counter,
                })),
                _ => {
                    return Err(ParseError {
                        msg: "A function has only one parameter but 2 were given".to_string(),
                        line: line_counter,
                    })
                }
            }
        }
        output.push(line_output);
        line_counter += 1;
    }
    if output[output.len() - 1] == vec![Token::End] {
        for line in output[0..output.len() - 1].iter() {
            if line.contains(&Token::End) {
                return Err(ParseError {
                    msg: format!("Multiple ends are not allowed"),
                    line: line_counter,
                });
            }
        }
        Ok((output, jumpmarks))
    } else {
        return Err(ParseError {
            msg: format!("The program has to end with an end"),
            line: line_counter,
        });
    }
}
