use crate::parser::Instruction;

fn is_same_instruction(instruction1: &Instruction, instruction2: &Instruction) -> bool {
    std::mem::discriminant(instruction1) == std::mem::discriminant(instruction2)
}

fn advance_loops(instructions: &mut Vec<Instruction>, index: usize, removed_count: usize) {
    for (instruction_index, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            Instruction::LoopStart(value) => {
                if index < *value {
                    *value -= removed_count;
                }
            }
            Instruction::LoopEnd(value) => {
                if index < *value && index < instruction_index {
                    *value -= removed_count;
                }
            }
            _ => (),
        }
    }
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

    instructions.drain(index + 1..index + 1 + count);
    advance_loops(instructions, index, count);
}

fn reset_value(instructions: &mut Vec<Instruction>, index: usize, value: usize) {
    if index + 2 != value {
        return;
    }

    match instructions[index + 1] {
        Instruction::IncrementValue(value) | Instruction::DecrementValue(value) => {
            if value == 1 {
                instructions[index] = Instruction::ResetValue;
                instructions.drain(index + 1..index + 3);
                advance_loops(instructions, index, 2);
            }
        }
        _ => (),
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
            Instruction::LoopStart(value) => reset_value(instructions, index, value),
            _ => (),
        }

        index += 1;
    }
}
