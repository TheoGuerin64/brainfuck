use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum Error {
    #[error("unclosed loop at line {0} char {1}")]
    NoLoopEnd(usize, usize),
    #[error("missing loop start at line {0} char {1}")]
    NoLoopStart(usize, usize),
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    IncrementPointer(usize),
    DecrementPointer(usize),
    IncrementValue(u8),
    DecrementValue(u8),
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

impl Instruction {
    pub fn from_char(char: char) -> Option<Instruction> {
        match char {
            '>' => Some(Instruction::IncrementPointer(1)),
            '<' => Some(Instruction::DecrementPointer(1)),
            '+' => Some(Instruction::IncrementValue(1)),
            '-' => Some(Instruction::DecrementValue(1)),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            '[' => Some(Instruction::LoopStart(0)),
            ']' => Some(Instruction::LoopEnd(0)),
            _ => None,
        }
    }
}

struct LoopStartIndex {
    index: usize,
    line_index: usize,
    char_index: usize,
}

pub fn parse(content: &str) -> Result<Vec<Instruction>, Error> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack: Vec<LoopStartIndex> = Vec::new();
    for (line_index, line) in content.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let Some(mut instruction) = Instruction::from_char(char) else {
                continue;
            };

            match &mut instruction {
                Instruction::LoopStart(_) => {
                    loop_stack.push(LoopStartIndex {
                        index: instructions.len(),
                        line_index: line_index + 1,
                        char_index: char_index + 1,
                    });
                }
                Instruction::LoopEnd(loop_start) => {
                    let Some(last) = loop_stack.pop() else {
                        return Err(Error::NoLoopStart(line_index + 1, char_index + 1));
                    };
                    instructions[last.index] = Instruction::LoopStart(instructions.len());
                    *loop_start = last.index;
                }
                _ => (),
            }

            instructions.push(instruction);
        }
    }
    if !loop_stack.is_empty() {
        return Err(Error::NoLoopEnd(
            loop_stack[0].line_index,
            loop_stack[0].char_index,
        ));
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
        let result: Vec<Instruction> = parse("__.+-__><--__,__").unwrap();
        let expected: Vec<Instruction> = vec![
            Instruction::Output,
            Instruction::IncrementValue(1),
            Instruction::DecrementValue(1),
            Instruction::IncrementPointer(1),
            Instruction::DecrementPointer(1),
            Instruction::DecrementValue(1),
            Instruction::DecrementValue(1),
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
            Instruction::DecrementValue(1),
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
            Instruction::IncrementPointer(1),
            Instruction::IncrementPointer(1),
            Instruction::LoopStart(11),
            Instruction::DecrementValue(1),
            Instruction::DecrementValue(1),
            Instruction::LoopStart(9),
            Instruction::LoopStart(8),
            Instruction::IncrementValue(1),
            Instruction::LoopEnd(6),
            Instruction::LoopEnd(5),
            Instruction::IncrementValue(1),
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
        let result = parse("__>>_[_--_[__+_]_]__+__]__+__");
        let Err(error) = result else {
            panic!("parsing missing start failed, value was {:?}", result);
        };
        assert!(
            error == Error::NoLoopStart(1, 24),
            "parsing missing start failed, value was {:?}",
            error
        );
    }

    #[test]
    fn parse_missing_end() {
        let result = parse("__>>_[_--__[_[__+_]_[_]__+__]__");
        let Err(error) = result else {
            panic!("parsing missing end failed, value was {:?}", result);
        };
        assert!(
            error == Error::NoLoopEnd(1, 6),
            "parsing missing end failed, value was {:?}",
            error
        );
    }
}
