use crate::parser::*;
use text_io::*;
use Token::*;


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

fn load(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32> ) {
    match token {
        Token::Number(x) => *akku = *x,
        Token::Address(x) => *akku = virtual_memory[ascii!(*x)],
        _ => panic!("something went wrong"),
    }
}

fn store(token: &Token, akku: &i32, virtual_memory: &mut Vec<i32>) {
    match token {
       Token::Address(x) => virtual_memory[ascii!(*x)] = *akku,
       _ => panic!("something went wrong"),
    }
}

fn add(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>) {
    match token {
       Token::Number(x) => *akku += x,
       Token::Address(x) => *akku += virtual_memory[ascii!(*x)],
       _ => panic!("something went wrong")
    }
}

fn sub(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>) {
    match token {
       Token::Number(x) => *akku -= x,
       Token::Address(x) => *akku -= virtual_memory[ascii!(*x)],
       _ => panic!("something went wrong")
    }
}

fn mul(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>) {
    match token {
       Token::Number(x) => *akku *= x,
       Token::Address(x) => *akku *= virtual_memory[ascii!(*x)],
       _ => panic!("something went wrong")
    }
}

fn div(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>) {
    match token {
       Token::Number(x) => *akku /= x,
       Token::Address(x) => *akku /= virtual_memory[ascii!(*x)],
       _ => panic!("something went wrong")
    }
}

fn modulo(token: &Token, akku: &mut i32, virtual_memory: &Vec<i32>) {
    match token {
       Token::Number(x) => *akku %= x,
       Token::Address(x) => *akku %= virtual_memory[ascii!(*x)],
       _ => panic!("something went wrong")
    }
}

fn input(token: &Token, virtual_memory: &mut Vec<i32>) {
    match token {
       Token::Address(x) => {
           let stdin: i32 = read!();
           virtual_memory[ascii!(*x)] = stdin;
       },
       _ => panic!("something went wrong"),
    }
}

fn output(token: &Token, virtual_memory: &Vec<i32>) {
    match token {
        Token::Number(x) => println!("{:?}", x),
        Token::Address(x) => println!("{:?}", virtual_memory[ascii!(*x)]),
        _ => panic!("something went wrong"),
    }
}

fn compare(token: &Token, virtual_memory: &Vec<i32>, akku: &i32, cmp_result: &mut CmpResult) {
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
        _ => panic!("something went wrong"),
    }
}

fn jump(token: &Token, pc: &mut usize) {
    match token {
       Token::Number(x) => {
           if *x >= 0 {
               *pc = *x as usize;
           } else {
               panic!("something went wrong");
           }
       },
       _ => panic!("something went wrong"),
    }
}

fn jump_if_less_then(token: &Token, pc: &mut usize, cmp_result: &CmpResult) {
    if *cmp_result == CmpResult::Less {
        jump(token, pc)
    }
}

fn jump_if_equal(token: &Token, pc: &mut usize, cmp_result: &CmpResult) {
    if *cmp_result == CmpResult::Equal {
        jump(token, pc)
    }
}

fn jump_if_greater_then(token: &Token, pc: &mut usize, cmp_result: &CmpResult) {
    if *cmp_result == CmpResult::Greater {
        jump(token, pc)
    }
}

pub fn run(input_programm: Vec<Vec<Token>>) {
    let mut virtual_memory = vec![0; 26];
    let mut akku: i32 = 0;
    let mut cmp_result: CmpResult = CmpResult::Equal;

    let mut pc = 0;
    loop {
        let line = &input_programm[pc];
        pc += 1;
        match line[0] {
            Ld => load(&line[1], &mut akku, &virtual_memory),
            St => store(&line[1], &akku, &mut virtual_memory),
            In => input(&line[1], &mut virtual_memory),
            Out => output(&line[1], &virtual_memory),
            Add => add(&line[1], &mut akku, &virtual_memory),
            Sub => sub(&line[1], &mut akku, &virtual_memory),
            Mul => mul(&line[1], &mut akku, &virtual_memory),
            Div => div(&line[1], &mut akku, &virtual_memory),
            Mod => modulo(&line[1], &mut akku, &virtual_memory),
            Cmp => compare(&line[1], &virtual_memory, &akku, &mut cmp_result),
            Jmp => jump(&line[1], &mut pc),
            Jlt => jump_if_less_then(&line[1], &mut pc, &cmp_result),
            Jeq => jump_if_equal(&line[1], &mut pc, &cmp_result),
            Jgt => jump_if_greater_then(&line[1], &mut pc, &cmp_result),
            End => break,
            _ => panic!("something went wrong"),
        }
    }
}
