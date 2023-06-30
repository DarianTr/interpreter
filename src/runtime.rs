use crate::parser::*;
use snafu::prelude::*;
use text_io::*;
use Token::*;

#[derive(Debug, Snafu)]
pub struct RuntimeError {
    pub msg: String,
    pub line: usize,
}

#[macro_export]
macro_rules! ascii {
    ($x:expr) => {
        ($x as u8 - 'a' as u8) as usize
    };
    ($x:expr, $y:expr) => {
        ($x as u8 - $y as u8) as usize
    };
}

#[derive(PartialEq, Eq)]
pub enum CmpResult {
    Greater,
    Equal,
    Less,
}

fn load(
    token: &Token,
    akku: &mut i32,
    virtual_memory: &Vec<i32>,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => *akku = *x,
        Token::Address(x) => *akku = virtual_memory[ascii!(*x)],
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot load {:?}", token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn store(
    token: &Token,
    akku: &i32,
    virtual_memory: &mut Vec<i32>,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Address(x) => virtual_memory[ascii!(*x)] = *akku,
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot store {:?} in {:?}", *akku, token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn add(
    token: &Token,
    akku: &mut i32,
    virtual_memory: &Vec<i32>,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => *akku += x,
        Token::Address(x) => *akku += virtual_memory[ascii!(*x)],
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot add {:?} with {:?}", *akku, token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn sub(
    token: &Token,
    akku: &mut i32,
    virtual_memory: &Vec<i32>,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => *akku -= x,
        Token::Address(x) => *akku -= virtual_memory[ascii!(*x)],
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot subtract {:?} with {:?}", *akku, token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn mul(
    token: &Token,
    akku: &mut i32,
    virtual_memory: &Vec<i32>,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => *akku *= x,
        Token::Address(x) => *akku *= virtual_memory[ascii!(*x)],
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot multiply {:?} with {:?}", *akku, token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn div(
    token: &Token,
    akku: &mut i32,
    virtual_memory: &Vec<i32>,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => *akku /= x,
        Token::Address(x) => *akku /= virtual_memory[ascii!(*x)],
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot divide {:?} by {:?}", *akku, token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn modulo(
    token: &Token,
    akku: &mut i32,
    virtual_memory: &Vec<i32>,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => *akku %= x,
        Token::Address(x) => *akku %= virtual_memory[ascii!(*x)],
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot calculate the modulo of {:?} and {:?}", *akku, token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn input(token: &Token, virtual_memory: &mut Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
        Token::Address(x) => {
            let stdin: i32 = read!();
            virtual_memory[ascii!(*x)] = stdin;
        }
        _ => {
            return Err(RuntimeError {
                msg: format!("the command \"In\" needs an address as parameter"),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn output(token: &Token, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => println!("{:?}", x),
        Token::Address(x) => println!("{:?}", virtual_memory[ascii!(*x)]),
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot output {:?}", token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn compare(
    token: &Token,
    virtual_memory: &Vec<i32>,
    akku: &i32,
    cmp_result: &mut CmpResult,
    pc: &usize,
) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => match akku.cmp(x) {
            std::cmp::Ordering::Less => *cmp_result = CmpResult::Less,
            std::cmp::Ordering::Equal => *cmp_result = CmpResult::Equal,
            std::cmp::Ordering::Greater => *cmp_result = CmpResult::Greater,
        },
        Token::Address(x) => match akku.cmp(&virtual_memory[ascii!(*x)]) {
            std::cmp::Ordering::Less => *cmp_result = CmpResult::Less,
            std::cmp::Ordering::Equal => *cmp_result = CmpResult::Equal,
            std::cmp::Ordering::Greater => *cmp_result = CmpResult::Greater,
        },
        _ => {
            return Err(RuntimeError {
                msg: format!("cannot compare {:?} and {:?}", *akku, token),
                line: *pc,
            })
        }
    }
    Ok(())
}

fn jump(token: &Token, pc: &mut usize, jumpmarklist: &Vec<Token>) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => {
            if *x >= 0 {
                *pc = *x as usize;
            } else {
                return Err(RuntimeError {
                    msg: format!("cannot jump to {:?}", token),
                    line: *pc,
                });
            }
        }
        Token::JumpMarkTo(x) => {
            let mut possible_destinations: Vec<usize> = vec![];
            for mark in jumpmarklist.iter() {
                match mark {
                    Token::Jumpmark(h) => {
                        if h.name == *x {
                            possible_destinations.push(h.line);
                        }
                    }
                    _ => {}
                }
            }
            if possible_destinations.len() > 1 {
                return Err(RuntimeError { msg: "ambiguous jump targe".to_owned(), line: *pc })
            } else if possible_destinations.is_empty() {
               return Err(RuntimeError { msg: "unknown jumpmark".to_owned(), line: *pc });
            } else {
                *pc = possible_destinations[0];
            }
        }
        _ => {
            return Err(RuntimeError {
                msg: format!("Something else made it into the list of all jumpmarks"),
                line: *pc,
            })
        }
    }

    Ok(())
}

fn jump_if_less_then(
    token: &Token,
    pc: &mut usize,
    cmp_result: &CmpResult,
    jumpmarklist: &Vec<Token>
) -> Result<(), RuntimeError> {
    if *cmp_result == CmpResult::Less {
        jump(token, pc, jumpmarklist)?
    }
    Ok(())
}

fn jump_if_equal(
    token: &Token,
    pc: &mut usize,
    cmp_result: &CmpResult,
    jumpmarklist: &Vec<Token>
) -> Result<(), RuntimeError> {
    if *cmp_result == CmpResult::Equal {
        jump(token, pc, jumpmarklist)?
    }
    Ok(())
}

fn jump_if_greater_then(
    token: &Token,
    pc: &mut usize,
    cmp_result: &CmpResult,
    jumpmarklist: &Vec<Token>
) -> Result<(), RuntimeError> {
    if *cmp_result == CmpResult::Greater {
        jump(token, pc, jumpmarklist)?
    }
    Ok(())
}

pub fn run(input_programm: Vec<Vec<Token>>, jumpmarklist: Vec<Token>) -> Result<(), RuntimeError> {
    let mut virtual_memory = vec![0; 26];
    let mut akku: i32 = 0;
    let mut cmp_result: CmpResult = CmpResult::Equal;

    let mut pc = 0;
    loop {
        let line = &input_programm[pc];
        pc += 1;
        match line[0] {
            Ld => load(&line[1], &mut akku, &virtual_memory, &pc)?,
            St => store(&line[1], &akku, &mut virtual_memory, &pc)?,
            In => input(&line[1], &mut virtual_memory, &pc)?,
            Out => output(&line[1], &virtual_memory, &pc)?,
            Add => add(&line[1], &mut akku, &virtual_memory, &pc)?,
            Sub => sub(&line[1], &mut akku, &virtual_memory, &pc)?,
            Mul => mul(&line[1], &mut akku, &virtual_memory, &pc)?,
            Div => div(&line[1], &mut akku, &virtual_memory, &pc)?,
            Mod => modulo(&line[1], &mut akku, &virtual_memory, &pc)?,
            Cmp => compare(&line[1], &virtual_memory, &akku, &mut cmp_result, &pc)?,
            Jmp => jump(&line[1], &mut pc, &jumpmarklist)?,
            Jlt => jump_if_less_then(&line[1], &mut pc, &cmp_result, &jumpmarklist)?,
            Jeq => jump_if_equal(&line[1], &mut pc, &cmp_result, &jumpmarklist)?,
            Jgt => jump_if_greater_then(&line[1], &mut pc, &cmp_result, &jumpmarklist)?,
            End => break,
            _ => {
                return Err(RuntimeError {
                    msg: format!("unknown command {:?}", line[0]),
                    line: pc,
                })
            }
        }
    }
    Ok(())
}
