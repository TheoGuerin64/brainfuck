use crate::parser::Instruction;

fn is_same_instruction(instruction1: &Instruction, instruction2: &Instruction) -> bool {
    std::mem::discriminant(instruction1) == std::mem::discriminant(instruction2)
}

fn merge_instructions(instructions: &mut Vec<Instruction>, index: usize) {
    let mut count: usize = 0;
    while instructions.len() >= index + count + 2
        && is_same_instruction(
            &instructions[index + count],
            &instructions[index + count + 1],
        )
    {
        count += 1;
    }

    match &mut instructions[index] {
        Instruction::IncrementValue(value) | Instruction::DecrementValue(value) => {
            *value += count as u8;
        }
        Instruction::IncrementPointer(value) | Instruction::DecrementPointer(value) => {
            *value += count;
        }
        _ => (),
    }

    for _ in 0..count {
        instructions.remove(index + 1);
    }

    for (instruction_index, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            Instruction::LoopStart(value) => {
                if index < *value {
                    *value -= count;
                }
            }
            Instruction::LoopEnd(value) => {
                if index < *value && index < instruction_index {
                    *value -= count;
                }
            }
            _ => (),
        }
    }
}

pub fn optimize(instructions: &mut Vec<Instruction>) {
    let mut index: usize = 0;
    while index < instructions.len() {
        match instructions[index] {
            Instruction::IncrementValue(_)
            | Instruction::DecrementValue(_)
            | Instruction::IncrementPointer(_)
            | Instruction::DecrementPointer(_) => merge_instructions(instructions, index),
            _ => (),
        }
        index += 1;
    }
}
