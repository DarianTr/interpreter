use crate::parser::*;
use text_io::*;
use Token::*;
use snafu::prelude::*;


#[derive(Debug, Snafu)]
#[snafu(display("Error in line {line}: {msg}"))]
pub struct RuntimeError {
    msg: String,
    line: usize,
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

fn load(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => *akku = *x,
        Token::Address(x) => *akku = virtual_memory[ascii!(*x)],
        _ => return Err(RuntimeError {
            msg: format!("cannot load {:?}", token),
            line: *pc,
        }
        )
    }
    Ok(())
}

fn store(token: &Token, akku: &i32, virtual_memory: &mut Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
       Token::Address(x) => virtual_memory[ascii!(*x)] = *akku,
       _ => return Err(RuntimeError {
           msg: format!("cannot store {:?} in {:?}", *akku, token),
           line: *pc,
       }),
    }
    Ok(())
}

fn add(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
       Token::Number(x) => *akku += x,
       Token::Address(x) => *akku += virtual_memory[ascii!(*x)],
       _ => return Err(RuntimeError { msg: format!("cannot add {:?} with {:?}", *akku, token), line: *pc })
    }
    Ok(())
}

fn sub(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
       Token::Number(x) => *akku -= x,
       Token::Address(x) => *akku -= virtual_memory[ascii!(*x)],
       _ => return Err(RuntimeError { msg: format!("cannot subtract {:?} with {:?}", *akku, token), line: *pc })
    }
    Ok(())
}

fn mul(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
       Token::Number(x) => *akku *= x,
       Token::Address(x) => *akku *= virtual_memory[ascii!(*x)],
       _ => return Err(RuntimeError { msg: format!("cannot multiply {:?} with {:?}", *akku, token), line: *pc })
    }
    Ok(())
}

fn div(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
       Token::Number(x) => *akku /= x,
       Token::Address(x) => *akku /= virtual_memory[ascii!(*x)],
       _ => return Err(RuntimeError { msg: format!("cannot divide {:?} by {:?}", *akku, token), line: *pc })
    }
    Ok(())
}

fn modulo(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
       Token::Number(x) => *akku %= x,
       Token::Address(x) => *akku %= virtual_memory[ascii!(*x)],
       _ => return Err(RuntimeError { msg: format!("cannot calculate the modulo of {:?} and {:?}", *akku, token), line: *pc})
    }
    Ok(())
}

fn input(token: &Token, virtual_memory: &mut Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
       Token::Address(x) => {
           let stdin: i32 = read!();
           virtual_memory[ascii!(*x)] = stdin;
       },
       _ => return Err(RuntimeError { msg: format!("the command \"In\" needs an address as parameter"), line: *pc }),
    }
    Ok(())
}

fn output(token: &Token, virtual_memory: &Vec<i32>, pc: &usize) -> Result<(), RuntimeError> {
    match token {
        Token::Number(x) => println!("{:?}", x),
        Token::Address(x) => println!("{:?}", virtual_memory[ascii!(*x)]),
        _ => return Err(RuntimeError { msg: format!("cannot output {:?}", token), line: *pc }),
    }
    Ok(())
}

fn compare(token: &Token, virtual_memory: &Vec<i32>, akku: &i32, cmp_result: &mut CmpResult, pc: &usize) -> Result<(), RuntimeError> {
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
        _ => return Err(RuntimeError { msg: format!("cannot compare {:?} and {:?}", *akku, token), line: *pc }),
    }
    Ok(())
}

fn jump(token: &Token, pc: &mut usize) -> Result<(), RuntimeError> {
    match token {
       Token::Number(x) => {
           if *x >= 0 {
               *pc = *x as usize;
           } else {
               panic!("something went wrong");
           }
       },
       _ => return Err(RuntimeError { msg: format!("cannot jump to {:?}", token), line: *pc }),
    }
    Ok(())
}

fn jump_if_less_then(token: &Token, pc: &mut usize, cmp_result: &CmpResult) -> Result<(), RuntimeError> {
    if *cmp_result == CmpResult::Less {
        jump(token, pc)?
    }
    Ok(())
}

fn jump_if_equal(token: &Token, pc: &mut usize, cmp_result: &CmpResult) -> Result<(), RuntimeError> {
    if *cmp_result == CmpResult::Equal {
        jump(token, pc)?
    }
    Ok(())
}

fn jump_if_greater_then(token: &Token, pc: &mut usize, cmp_result: &CmpResult) -> Result<(), RuntimeError> {
    if *cmp_result == CmpResult::Greater {
        jump(token, pc)?
    }
    Ok(())
}

pub fn run(input_programm: Vec<Vec<Token>>) -> Result<(), RuntimeError> {
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
            Jmp => jump(&line[1], &mut pc)?,
            Jlt => jump_if_less_then(&line[1], &mut pc, &cmp_result)?,
            Jeq => jump_if_equal(&line[1], &mut pc, &cmp_result)?,
            Jgt => jump_if_greater_then(&line[1], &mut pc, &cmp_result)?,
            End => break,
            _ => return Err(RuntimeError { msg: format!("unknown command {:?}", line[0]), line: pc }),
        }
    }
    Ok(())
}
