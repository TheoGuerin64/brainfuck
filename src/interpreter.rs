use crate::parser::Instruction;
use std::num::Wrapping;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("out of bounds")]
    OutOfBounds,
}

fn move_pointer(pointer: &mut usize, increment: bool) -> Result<(), Error> {
    if increment {
        if *pointer >= 30_000 - 1 {
            return Err(Error::OutOfBounds);
        }
        *pointer += 1;
    } else {
        if *pointer == 0 {
            return Err(Error::OutOfBounds);
        }
        *pointer -= 1;
    }
    Ok(())
}

fn input_ascii() -> u8 {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        let Ok(size) = std::io::stdin().read_line(&mut buffer) else {
            eprintln!("input error");
            continue;
        };

        if size == 0 {
            return 0;
        }
        if size != 2 {
            eprintln!("input must be a single character");
            continue;
        }

        let raw_char = buffer.chars().next().unwrap();
        if !raw_char.is_ascii() {
            eprintln!("input must be an ascii character");
            continue;
        }

        return raw_char as u8;
    }
}

pub fn execute(instructions: &Vec<Instruction>) -> Result<(), Error> {
    let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
    let mut pointer: usize = 0;
    let mut line: usize = 0;
    while line < instructions.len() {
        match instructions[line] {
            Instruction::IncrementPointer => move_pointer(&mut pointer, true)?,
            Instruction::DecrementPointer => move_pointer(&mut pointer, false)?,
            Instruction::IncrementValue => memory[pointer] += 1,
            Instruction::DecrementValue => memory[pointer] -= 1,
            Instruction::Output => print!("{}", memory[pointer].0 as char),
            Instruction::Input => {
                memory[pointer] = Wrapping(input_ascii());
            }
            Instruction::LoopStart(loop_end) => {
                if memory[pointer].0 == 0 {
                    line = loop_end;
                }
            }
            Instruction::LoopEnd(loop_start) => {
                if memory[pointer].0 != 0 {
                    line = loop_start;
                }
            }
        }
        line += 1;
    }
    Ok(())
}
