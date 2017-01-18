use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;


pub fn run<P, R, W>(code_path: P, input: R, output: &mut W)
    where P: AsRef<Path>, R: Read, W: Write
{
    let code_file = File::open(code_path).unwrap();
    let code: Vec<char> = code_file.bytes()
        .map(|byte_result| byte_result.unwrap() as char)
        .collect();
    let mut code_pointer = 0;

    let mut input = input.bytes();

    let mut tape: Vec<u8> = vec![0];
    let mut tape_pointer = 0;

    while code_pointer < code.len() {
        match code[code_pointer] {
            '>' => {
                tape_pointer += 1;
                if tape_pointer >= tape.len() {
                    tape.push(0)
                }
            }
            '<' => tape_pointer -= 1,
            '+' => tape[tape_pointer] = tape[tape_pointer].wrapping_add(1),
            '-' => tape[tape_pointer] = tape[tape_pointer].wrapping_sub(1),
            '.' => {
                output.write_all(&[tape[tape_pointer]]).unwrap();
                output.flush().unwrap()
            },
            ',' => tape[tape_pointer] = match input.next() {
                Some(x) => x.unwrap(),
                None    => 0,
            },
            '[' => {
                if tape[tape_pointer] == 0 {
                    let mut count = 1;
                    while count > 0 {
                        code_pointer += 1;
                        if code[code_pointer] == '[' {
                            count += 1
                        } else if code[code_pointer] == ']' {
                            count -= 1
                        }
                    }
                }
            }
            ']' => {
                if tape[tape_pointer] != 0 {
                    let mut count = 1;
                    while count > 0 {
                        code_pointer -= 1;
                        if code[code_pointer] == '[' {
                            count -= 1
                        } else if code[code_pointer] == ']' {
                            count += 1
                        }
                    }
                }
            }
            _ => (),
        }
        code_pointer += 1;
    }
}
