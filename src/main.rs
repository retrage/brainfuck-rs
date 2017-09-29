use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::process;

enum BfOpKind {
    InvalidOp,
    IncPtr,
    DecPtr,
    IncData,
    DecData,
    ReadStdin,
    WriteStdout,
    JumpIfDataZero,
    JumpIfDataNotZero,
}

#[allow(dead_code)]
fn bf_op_kind_name(kind: BfOpKind) -> char {
    match kind {
        BfOpKind::IncPtr => return '>',
        BfOpKind::DecPtr => return '<',
        BfOpKind::IncData => return '+',
        BfOpKind::DecData => return '-',
        BfOpKind::ReadStdin => return ',',
        BfOpKind::WriteStdout => return '.',
        BfOpKind::JumpIfDataZero => return '[',
        BfOpKind::JumpIfDataNotZero => return ']',
        BfOpKind::InvalidOp => return 'x',
    }
}

struct BfOp {
    kind: BfOpKind,
    argument: u64,
}

fn translate_code(code: &Vec<u8>) -> Vec<BfOp> {
    let mut ptr = 0;
    let code_size = code.len();
    let mut ops =  Vec::new();
    let mut open_bracket_stack = Vec::new();

    while ptr < code_size {
        let instruction = code[ptr];
        if instruction == '[' as u8 {
            open_bracket_stack.push(ops.len() as u64);
            ops.push(BfOp{ kind: BfOpKind::JumpIfDataZero,
                            argument: 0 });
            ptr += 1;
        } else if instruction == ']' as u8 {
            if open_bracket_stack.is_empty() {
                panic!("Unmatch closing ']': ptr={}", ptr);
            }
            let open_bracket_offset =
                    open_bracket_stack[open_bracket_stack.len()-1];
            open_bracket_stack.pop();

            ops[open_bracket_offset as usize].argument = ops.len() as u64;
            ops.push(BfOp{ kind:BfOpKind::JumpIfDataNotZero, 
                            argument: open_bracket_offset });
            ptr += 1;
        } else {
            let start = ptr;
            ptr += 1;
            while ptr < code_size && code[ptr] == instruction {
                ptr += 1;
            }

            let num_repeats = ptr - start;

            let mut kind = BfOpKind::InvalidOp;
            match instruction as char {
                '>' => kind = BfOpKind::IncPtr,
                '<' => kind = BfOpKind::DecPtr,
                '+' => kind = BfOpKind::IncData,
                '-' => kind = BfOpKind::DecData,
                ',' => kind = BfOpKind::ReadStdin,
                '.' => kind = BfOpKind::WriteStdout,
                _ => {}
            }

            ops.push(BfOp{ kind: kind, argument: num_repeats as u64 });
        }
    }

    ops
}

fn input() -> u8 {
    let c = match std::io::stdin().bytes().next()
                .and_then(|result| result.ok()).map(|byte| byte as u8) {
        None => process::exit(0),
        Some(c) => c,
    };
    c
}

fn output(c: u8) {
    print!("{}", c as char);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}",
                           display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}: {}",
                           display, Error::description(&why)),
        Ok(_) => {},
    }

    let mut code_ptr: usize = 0;
    let mut data_ptr: usize = 0;
    let code = s.into_bytes();
    let mut data = [0 as u8; 30000];
    let ops = translate_code(&code);

    while code_ptr < ops.len() {
        let op = &ops[code_ptr];
        let kind = &op.kind;

        match kind {
            &BfOpKind::IncPtr => data_ptr += op.argument as usize,
            &BfOpKind::DecPtr => data_ptr -= op.argument as usize,
            &BfOpKind::IncData => data[data_ptr] += op.argument as u8,
            &BfOpKind::DecData => data[data_ptr] -= op.argument as u8,
            &BfOpKind::ReadStdin => {
                for _ in 0..op.argument {
                    data[data_ptr] = input();
                }
            },
            &BfOpKind::WriteStdout => {
                for _ in 0..op.argument {
                    output(data[data_ptr]);
                }
            },
            &BfOpKind::JumpIfDataZero => {
                if data[data_ptr] == 0 {
                    code_ptr = op.argument as usize;
                }
            },
            &BfOpKind::JumpIfDataNotZero => {
                if data[data_ptr] != 0 {
                    code_ptr = op.argument as usize;
                }
            },
            _ => {},
        }
        code_ptr += 1;
    }
}
