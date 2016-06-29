use std::io;
use std::vec::Vec;
use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = io::stdin().bytes();

    let code_file = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let code: Vec<_> = code_file.bytes()
        .map(|byte_result| byte_result.unwrap() as char)
        .collect();
    let mut code_pointer = 0;

    let mut tape = vec![0];
    let mut tape_pointer = 0;

    while code_pointer < code.len() {
        match code[code_pointer] {
            '>' => {
                tape_pointer += 1;
                if tape_pointer >= tape.len() {
                    tape.push(0)
                }
            },
            '<' => tape_pointer -= 1,
            '+' => tape[tape_pointer] += 1,
            '-' => tape[tape_pointer] -= 1,
            '.' => print!("{}", tape[tape_pointer] as char),
            ',' => tape[tape_pointer] = input.next().unwrap().unwrap(),
            '[' => if tape[tape_pointer] == 0 {
                let mut count = 1;
                while count > 0 {
                    code_pointer += 1;
                    if code[code_pointer] == '[' {
                        count += 1
                    } else if code[code_pointer] == ']' {
                        count -= 1
                    }
                }
                return
            },
            ']' => if tape[tape_pointer] != 0 {
                let mut count = 1;
                while count > 0 {
                    code_pointer -= 1;
                    if code[code_pointer] == '[' {
                        count -= 1
                    } else if code[code_pointer] == ']' {
                        count += 1
                    }
                }
            },
            _ => ()
        }
        code_pointer += 1;
    }
}
