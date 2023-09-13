mod optimizer;
mod parser;

use parser::Instruction;
use std::fs;
use std::num::Wrapping;
use std::process;
use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum Error {
    #[error("out of bounds")]
    OutOfBounds,
}

fn move_pointer(pointer: &mut usize, value: usize, increment: bool) -> Result<(), Error> {
    if increment {
        let Some(new_value) = pointer.checked_add(value) else {
            return Err(Error::OutOfBounds);
        };
        if new_value >= 30_000 {
            return Err(Error::OutOfBounds);
        }
        *pointer = new_value;
    } else {
        let Some(new_value) = pointer.checked_sub(value) else {
            return Err(Error::OutOfBounds);
        };
        *pointer = new_value;
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

fn execute_instruction(
    memory: &mut [Wrapping<u8>],
    pointer: &mut usize,
    index: &mut usize,
    instruction: &Instruction,
) -> Result<(), Error> {
    match instruction {
        Instruction::IncrementPointer(value) => move_pointer(pointer, *value, true)?,
        Instruction::DecrementPointer(value) => move_pointer(pointer, *value, false)?,
        Instruction::IncrementValue(value) => memory[*pointer] += *value,
        Instruction::DecrementValue(value) => memory[*pointer] -= *value,
        Instruction::Output => print!("{}", memory[*pointer].0 as char),
        Instruction::Input => memory[*pointer] = Wrapping(input_ascii()),
        Instruction::LoopStart(loop_end) => {
            if memory[*pointer].0 == 0 {
                *index = *loop_end;
            }
        }
        Instruction::LoopEnd(loop_start) => {
            if memory[*pointer].0 != 0 {
                *index = *loop_start;
            }
        }
        Instruction::ResetValue => memory[*pointer] = Wrapping(0),
    }
    Ok(())
}

fn execute(instructions: &Vec<Instruction>) -> Result<(), Error> {
    let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
    let mut pointer: usize = 0;
    let mut index: usize = 0;
    while index < instructions.len() {
        let instruction = &instructions[index];
        execute_instruction(&mut memory, &mut pointer, &mut index, instruction)?;
        index += 1;
    }
    Ok(())
}

pub fn interpret(file_path: &str) {
    let content = fs::read_to_string(file_path).unwrap_or_else(|error| {
        eprintln!("file reading error: {error}");
        process::exit(1);
    });
    let mut instructions = parser::parse(&content).unwrap_or_else(|error| {
        eprintln!("parsing error: {error}");
        process::exit(1);
    });
    optimizer::optimize(&mut instructions);
    if let Err(error) = execute(&instructions) {
        eprintln!("execution error: {error}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_increment_value() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 0;
        let mut line: usize = 0;

        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::IncrementValue(1),
        );

        if let Err(error) = result {
            panic!("increment value failed, value was {:?}", error);
        }
        assert!(
            memory[0] == Wrapping(1),
            "increment value failed, value was {:?}",
            memory[0]
        );
    }

    #[test]
    fn execute_increment_value_overflow() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 0;
        let mut line: usize = 0;

        memory[0] = Wrapping(255);
        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::IncrementValue(1),
        );

        if let Err(error) = result {
            panic!("increment value overflow failed, value was {:?}", error);
        }
        assert!(
            memory[0] == Wrapping(0),
            "increment value overflow failed, value was {:?}",
            memory[0]
        );
    }

    #[test]
    fn execute_decrement_value() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 0;
        let mut line: usize = 0;

        memory[0] = Wrapping(1);
        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::DecrementValue(1),
        );

        if let Err(error) = result {
            panic!("decrement value failed, value was {:?}", error);
        }
        assert!(
            memory[0] == Wrapping(0),
            "decrement value failed, value was {:?}",
            memory[0]
        );
    }

    #[test]
    fn execute_decrement_value_overflow() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 0;
        let mut line: usize = 0;

        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::DecrementValue(1),
        );

        if let Err(error) = result {
            panic!("decrement value overflow failed, value was {:?}", error);
        }
        assert!(
            memory[0] == Wrapping(255),
            "decrement value overflow failed, value was {:?}",
            memory[0]
        );
    }

    #[test]
    fn execute_increment_pointer() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 0;
        let mut line: usize = 0;

        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::IncrementPointer(1),
        );

        if let Err(error) = result {
            panic!("increment pointer failed, value was {:?}", error);
        }
        assert!(
            pointer == 1,
            "increment pointer failed, value was {:?}",
            pointer
        );
    }

    #[test]
    fn execute_increment_pointer_out_of_bounds() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 30_000 - 1;
        let mut line: usize = 0;

        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::IncrementPointer(1),
        );

        let Err(error) = result else {
            panic!(
                "increment pointer out of bounds failed, value was {:?}",
                result
            );
        };
        assert!(
            error == Error::OutOfBounds,
            "increment pointer out of bounds failed, value was {:?}",
            error
        );
    }

    #[test]
    fn execute_decrement_pointer() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 1;
        let mut line: usize = 0;

        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::DecrementPointer(1),
        );

        if let Err(error) = result {
            panic!("decrement pointer failed, value was {:?}", error);
        }
        assert!(
            pointer == 0,
            "decrement pointer failed, value was {:?}",
            pointer
        );
    }

    #[test]
    fn execute_decrement_pointer_out_of_bounds() {
        let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
        let mut pointer: usize = 0;
        let mut line: usize = 0;

        let result = execute_instruction(
            &mut memory,
            &mut pointer,
            &mut line,
            &Instruction::DecrementPointer(1),
        );

        let Err(error) = result else {
            panic!(
                "decrement pointer out of bounds failed, value was {:?}",
                result
            );
        };
        assert!(
            error == Error::OutOfBounds,
            "decrement pointer out of bounds failed, value was {:?}",
            error
        );
    }
}
