use crate::parser::Instruction;
use std::num::Wrapping;
use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
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

fn execute_instruction(
    memory: &mut [Wrapping<u8>],
    pointer: &mut usize,
    line: &mut usize,
    instruction: &Instruction,
) -> Result<(), Error> {
    match instruction {
        Instruction::IncrementPointer => move_pointer(pointer, true)?,
        Instruction::DecrementPointer => move_pointer(pointer, false)?,
        Instruction::IncrementValue => memory[*pointer] += 1,
        Instruction::DecrementValue => memory[*pointer] -= 1,
        Instruction::Output => print!("{}", memory[*pointer].0 as char),
        Instruction::Input => memory[*pointer] = Wrapping(input_ascii()),
        Instruction::LoopStart(loop_end) => {
            if memory[*pointer].0 == 0 {
                *line = *loop_end;
            }
        }
        Instruction::LoopEnd(loop_start) => {
            if memory[*pointer].0 != 0 {
                *line = *loop_start;
            }
        }
    }
    Ok(())
}

pub fn execute(instructions: &Vec<Instruction>) -> Result<(), Error> {
    let mut memory: [Wrapping<u8>; 30_000] = [Wrapping(0); 30_000];
    let mut pointer: usize = 0;
    let mut line: usize = 0;
    while line < instructions.len() {
        let instruction = &instructions[line];
        execute_instruction(&mut memory, &mut pointer, &mut line, instruction)?;
        line += 1;
    }
    Ok(())
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
            &Instruction::IncrementValue,
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
            &Instruction::IncrementValue,
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
            &Instruction::DecrementValue,
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
            &Instruction::DecrementValue,
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
            &Instruction::IncrementPointer,
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
            &Instruction::IncrementPointer,
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
            &Instruction::DecrementPointer,
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
            &Instruction::DecrementPointer,
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
