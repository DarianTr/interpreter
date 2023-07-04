use text_io::read;
use snafu::Snafu;

#[derive(PartialEq, Eq, Debug)]
pub enum CmpResult {
    Greater,
    Equal,
    Less,
}

// decode token to usize:
// 0: End
// 1: In
// 2: St
// 3: Ld (address)
// 4: Ld (number)
// 5: Add (address)
// 6: Add (number)
// 7: Sub (address)
// 8: Sub (number)
// 9: Mul (address)
// 10: Mul (number)
// 11: Div (address)
// 12: Div (number)
// 13: Mod (address)
// 14: Mod (number)
// 15: Cmp (address)
// 16: Cmp (number)
// 17: Out (address)
// 18: Out (number)
// 19: Jmp (number)
// 19.5: Jmp (jumpmark) has to be translated to number
// 20: Jlt
// 21: Jeq
// 22: Jgt

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

pub fn run(memory: &mut Vec<i32>) -> Result<(), RuntimeError>  {
    let mut pc: usize = 0;
    let mut akku = 0;
    let mut cmp_flag = CmpResult::Equal;
    loop {
        match memory[2 * pc] {
            0 => break,
            1 => {
                let stdin: i32 = read!();
                let idx = memory[2 * pc + 1] as usize;
                memory[idx] = stdin;
            },
            2 => {
                let idx = memory[2 * pc + 1] as usize;
                memory[idx] = akku;
            },
            3 => akku = memory[memory[2 * pc + 1] as usize],
            4 => akku = memory[2 * pc + 1],
            5 => akku += memory[memory[2 * pc + 1] as usize],
            6 => akku += memory[2 * pc + 1],
            7 => akku -= memory[memory[2 * pc + 1] as usize],
            8 => akku -= memory[2 * pc + 1],
            9 => akku *= memory[memory[2 * pc + 1] as usize],
            10 => akku *= memory[2 * pc + 1],
            11 => akku /= memory[memory[2 * pc + 1] as usize],
            12 => akku /= memory[2 * pc + 1],
            13 => akku %= memory[memory[2 * pc + 1] as usize],
            14 => akku %= memory[2 * pc + 1],
            15 => compare(akku, memory[memory[2 * pc + 1] as usize], &mut cmp_flag),
            16 => compare(akku, memory[2 * pc + 1], &mut cmp_flag),
            17 => println!("{:?}", memory[memory[2 * pc + 1] as usize]),
            18 => println!("{:?}", memory[2 * pc + 1]),
            19 => pc = memory[2 * pc + 1] as usize - 1,
            20 => if cmp_flag == CmpResult::Less {pc = memory[2 * pc + 1] as usize - 1},
            21 => if cmp_flag == CmpResult::Equal {pc = memory[2 * pc + 1] as usize - 1},
            22 => if cmp_flag == CmpResult::Greater {pc = memory[2 * pc + 1] as usize - 1},
            _ => return Err(RuntimeError { msg: "unknown function".to_owned(), line: pc })

        }
        pc += 1;
    }
    Ok(())
}

fn compare(a: i32, b: i32, cmp_result: &mut CmpResult) {
    match a.cmp(&b) {
            std::cmp::Ordering::Less => *cmp_result = CmpResult::Less,
            std::cmp::Ordering::Equal => *cmp_result = CmpResult::Equal,
            std::cmp::Ordering::Greater => *cmp_result = CmpResult::Greater,
    }
}
