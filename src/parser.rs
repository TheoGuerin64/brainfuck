use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum Error {
    #[error("unclosed loop")]
    NoLoopEnd,
    #[error("missing loop start")]
    NoLoopStart,
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

impl Instruction {
    pub fn from_char(char: char) -> Option<Instruction> {
        match char {
            '>' => Some(Instruction::IncrementPointer),
            '<' => Some(Instruction::DecrementPointer),
            '+' => Some(Instruction::IncrementValue),
            '-' => Some(Instruction::DecrementValue),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            '[' => Some(Instruction::LoopStart(0)),
            ']' => Some(Instruction::LoopEnd(0)),
            _ => None,
        }
    }
}

pub fn parse(content: &str) -> Result<Vec<Instruction>, Error> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    for char in content.chars() {
        let Some(mut instruction) = Instruction::from_char(char) else {
            continue;
        };

        match &mut instruction {
            Instruction::LoopStart(_) => {
                loop_stack.push(instructions.len());
            }
            Instruction::LoopEnd(loop_start) => {
                let Some(last) = loop_stack.pop() else {
                    return Err(Error::NoLoopStart);
                };
                instructions[last] = Instruction::LoopStart(instructions.len());
                *loop_start = last;
            }
            _ => (),
        }

        instructions.push(instruction);
    }
    if !loop_stack.is_empty() {
        return Err(Error::NoLoopEnd);
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_comments() {
        let result = parse("d{{Q\\' =(L^/6;/B\"D;;=%? joY=SA").unwrap();
        assert!(
            result.is_empty(),
            "parsing comments failed, value was {:?}",
            result
        );
    }

    #[test]
    fn parse_basic() {
        let result: Vec<Instruction> = parse("__.++__><--__,__").unwrap();
        let expected: Vec<Instruction> = vec![
            Instruction::Output,
            Instruction::IncrementValue,
            Instruction::IncrementValue,
            Instruction::IncrementPointer,
            Instruction::DecrementPointer,
            Instruction::DecrementValue,
            Instruction::DecrementValue,
            Instruction::Input,
        ];
        assert!(
            result.eq(&expected),
            "parsing basic failed, value was {:?}",
            result
        );
    }

    #[test]
    fn parse_loop() {
        let result: Vec<Instruction> = parse("__,__[__-_.__]__").unwrap();
        let expected: Vec<Instruction> = vec![
            Instruction::Input,
            Instruction::LoopStart(4),
            Instruction::DecrementValue,
            Instruction::Output,
            Instruction::LoopEnd(1),
        ];
        assert!(
            result.eq(&expected),
            "parsing basic failed, value was {:?}",
            result
        );
    }

    #[test]
    fn parse_nested_loops() {
        let result: Vec<Instruction> = parse("__>>_[_--__[_[__+_]_]__+__]__").unwrap();
        let expected: Vec<Instruction> = vec![
            Instruction::IncrementPointer,
            Instruction::IncrementPointer,
            Instruction::LoopStart(11),
            Instruction::DecrementValue,
            Instruction::DecrementValue,
            Instruction::LoopStart(9),
            Instruction::LoopStart(8),
            Instruction::IncrementValue,
            Instruction::LoopEnd(6),
            Instruction::LoopEnd(5),
            Instruction::IncrementValue,
            Instruction::LoopEnd(2),
        ];
        assert!(
            result.eq(&expected),
            "parsing nested loops failed, value was {:?}",
            result
        );
    }

    #[test]
    fn parse_missing_start() {
        let result = parse("__>>_[_--_[__+_]_]__+__]__");
        assert!(
            result.is_err_and(|e| e == Error::NoLoopStart),
            "parsing missing start should fail with NoLoopStart"
        );
    }

    #[test]
    fn parse_missing_end() {
        let result = parse("__>>_[_--__[_[__+_]_[_]__+__]__");
        assert!(
            result.is_err_and(|e| e == Error::NoLoopEnd),
            "parsing missing end should fail with NoLoopEnd"
        );
    }
}
