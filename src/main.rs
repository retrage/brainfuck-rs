use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::process;

fn compute_jumptable(code: &Vec<u8>) -> Vec<u64> {
    let mut ptr = 0;
    let mut jumptable = vec![0 as u64; code.len()];
    let mut bracket_nesting =  0;
    let mut seek = ptr;

    while ptr < code.len() {
        if code[ptr] == '[' as u8 {
            bracket_nesting = 1;
            seek = ptr;

            while bracket_nesting != 0 && (seek+1 < code.len()) {
                seek += 1;
                if code[seek] == ']' as u8 {
                    bracket_nesting -= 1;
                } else if code[seek] == '[' as u8 {
                    bracket_nesting += 1;
                }
            }

            if bracket_nesting == 0 {
                jumptable[ptr] = seek as u64;
                jumptable[seek] = ptr as u64;
            } else {
                panic!("Unmatched '[': ptr={}", ptr);
            }
        }
        ptr += 1;
    }

    jumptable
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
    let jumptable = compute_jumptable(&code);

    while code_ptr < code.len() {
        let c = code[code_ptr];
        match c as char {
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '+' => data[data_ptr] += 1,
            '-' => data[data_ptr] -= 1,
            '.' => output(data[data_ptr]),
            ',' => data[data_ptr] = input(),
            '[' => {
                if data[data_ptr] == 0 {
                    code_ptr = jumptable[code_ptr] as usize;
                }
            },
            ']' => {
                if data[data_ptr] != 0 {
                    code_ptr = jumptable[code_ptr] as usize;
                }
            },
            _ => {},
        }
        code_ptr += 1;
    }
}
