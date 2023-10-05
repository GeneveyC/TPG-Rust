//! # Program
//! Crate that permit to execute a program of the Tangled Program Graph (TPG).

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use crate::{flip, ProgramParams};
use rand::Rng;

#[derive(Clone, Debug)]
/// Structure that represent the instruction.
pub struct Instruction {
    /// The mode used by the instruction.
    /// If 0
    ///     |-> The instruction take data from the register.
    /// If 1
    ///     |-> The instrucetion take data from the input.
    mode: i32,
    /// The operation made by the instruction.
    op: i32,
    /// The index of the data inpût to take (in the register or the input).
    src: i32,
    /// The index of the destination.
    dst: i32,
}

impl PartialEq for Instruction {
    /// Function to compare two instruction between them.
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.op == other.op
            && self.src == other.src
            && self.dst == other.dst
    }
}

impl Instruction {
    /// Constructor of the instruction.
    pub fn new(mode: i32, op: i32, src: i32, dst: i32) -> Self {
        Instruction { mode, op, src, dst }
    }
}

#[derive(Clone, Debug)]
/// Structure that represent the program.
pub struct Program {
    /// The id of the program.
    id: i32,
    /// The list of instruction inside the program.
    instructions: Vec<Instruction>,
}

impl PartialEq for Program {
    /// Function to compare two program between them.
    fn eq(&self, other: &Self) -> bool {
        self.instructions.len() == other.instructions.len()
            && self.instructions == other.instructions
    }
}

impl Program {
    /// Constructor of the program.
    pub fn new(
        instructions_heritage: Option<Vec<Instruction>>,
        init_params: &mut ProgramParams,
    ) -> Self {
        let id: i32 = init_params.get_new_id_program();
        let mut instructions: Vec<Instruction> = Vec::new();

        if let Some(instruction_parents) = instructions_heritage {
            instructions = instruction_parents;
        } else {
            let mut rng = rand::thread_rng();

            for _ in 0..init_params.max_program_length {
                let mode: i32 = rng.gen_range(0..2);
                let op: i32 = rng.gen_range(0..init_params.nb_operations);
                let src: i32 = rng.gen_range(0..init_params.input_size);
                let dst: i32 = rng.gen_range(0..init_params.nb_destinations);

                let inst: Instruction = Instruction::new(mode, op, src, dst);
                instructions.push(inst);
            }
        }

        Program { id, instructions }
    }

    /// Function to clear the instruction inside the program.
    pub fn reset(&mut self) {
        self.instructions.clear();
    }

    /// Function to set a new instruction inside the poll of instruction.
    pub fn set_instructions(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Function to get the id of the program.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Function to get the number of instruction inside the program
    pub fn get_len_program(&self) -> usize {
        self.instructions.len()
    }

    /// Function to execute a program
    pub fn execute(&self, input: &Vec<i32>, regs: &mut Vec<i32>) {
        let input_len: i32 = input.len().try_into().unwrap();
        let regs_len: i32 = regs.len().try_into().unwrap();

        for instruction in self.instructions.iter() {
            let mut src = input[(instruction.src % input_len) as usize];
            if instruction.mode == 0 {
                src = regs[(instruction.src % regs_len) as usize];
            }

            let op = instruction.op;

            let x = regs[(instruction.dst) as usize];
            let y = src;
            let dest = (instruction.dst % regs_len) as usize;

            if op == 0 {
                regs[dest] = x + y;
            } else if op == 1 {
                regs[dest] = x - y;
            } else if op == 2 {
                regs[dest] = x * 2;
            } else if op == 3 {
                regs[dest] = x / 2;
            } else if op == 4 && x < y {
                regs[dest] = -x;
            }
        }
    }

    /// Function to mutate the program
    pub fn mutate(&mut self, mutate_params: &ProgramParams) {
        let mut rng = rand::thread_rng();

        let original_instruction = self.instructions.clone();
        let mut current_instruction = self.instructions.clone();

        while current_instruction == original_instruction {
            if self.instructions.len() > 1 && flip(mutate_params.p_inst_del) {
                println!("[Mutation] Instruction delete...");
                let index = rng.gen_range(0..self.instructions.len());
                self.instructions.remove(index);
            }

            if flip(mutate_params.p_inst_mut) {
                println!("[Mutation] Instruction mutate...");
                let index1 = rng.gen_range(0..self.instructions.len());
                let index2 = rng.gen_range(0..4);

                if index2 == 0 {
                    let max_value: i32 = rng.gen_range(0..2);
                    self.instructions[index1].mode = max_value;
                } else if index2 == 1 {
                    let max_value: i32 = rng.gen_range(0..mutate_params.nb_operations);
                    self.instructions[index1].op = max_value;
                } else if index2 == 2 {
                    let max_val: i32 = rng.gen_range(0..mutate_params.nb_destinations);
                    self.instructions[index1].dst = max_val;
                } else if index2 == 3 {
                    let max_val: i32 = rng.gen_range(0..mutate_params.input_size);
                    self.instructions[index1].src = max_val;
                }
            }

            if flip(mutate_params.p_inst_swap) {
                println!("[Mutation] Instruction swap...");
                let index1 = rng.gen_range(0..self.instructions.len());
                let mut index2 = rng.gen_range(0..self.instructions.len());

                while index2 == index1 {
                    index2 = rng.gen_range(0..self.instructions.len());
                }

                self.instructions.swap(index1, index2);
            }

            if flip(mutate_params.p_inst_add) {
                println!("[Mutation] Instruction add...");
                let mode: i32 = rng.gen_range(0..2);
                let op: i32 = rng.gen_range(0..mutate_params.nb_operations);
                let src: i32 = rng.gen_range(0..mutate_params.input_size);
                let dst: i32 = rng.gen_range(0..mutate_params.nb_destinations);

                let inst: Instruction = Instruction::new(mode, op, src, dst);
                self.instructions.push(inst);
            }

            current_instruction = self.instructions.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProgramParams;

    #[test]
    fn test_init_program() {
        let mut init_params: ProgramParams = ProgramParams {
            id_counter_program: 47,
            max_program_length: 5,
            nb_operations: 5,
            input_size: 3,
            nb_destinations: 3,
            p_inst_del: 0.0,
            p_inst_mut: 0.0,
            p_inst_swap: 0.0,
            p_inst_add: 0.0,
        };
        let mut p1: Program = Program::new(None, &mut init_params);

        // Check the id of program
        assert_eq!(
            p1.get_id(),
            47,
            "The id of the program is not the same as defined"
        );

        // Check the number of instruction after init
        assert_eq!(
            p1.get_len_program(),
            5,
            "The number of instruction is incorrect after init"
        );

        // Check the number of instruction after reset
        p1.reset();
        assert_eq!(
            p1.get_len_program(),
            0,
            "The number of instruction is incorrect after reset"
        );

        // Check the number of instruction after set
        let i1: Instruction = Instruction::new(1, 0, 0, 0);
        p1.set_instructions(i1);
        assert_eq!(
            p1.get_len_program(),
            1,
            "The number of instruction is incorrect after set instruction"
        );
    }

    #[test]
    fn test_execute_program() {
        let mut init_params: ProgramParams = ProgramParams {
            id_counter_program: 47,
            max_program_length: 5,
            nb_operations: 5,
            input_size: 3,
            nb_destinations: 3,
            p_inst_del: 0.0,
            p_inst_mut: 0.0,
            p_inst_swap: 0.0,
            p_inst_add: 0.0,
        };
        let mut p1: Program = Program::new(None, &mut init_params);

        // regs[0] = input[0] + regs[0]
        let i1: Instruction = Instruction::new(1, 0, 0, 0);

        p1.reset();
        p1.set_instructions(i1);

        let input: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut regs: Vec<i32> = vec![6, 7, 8, 9, 10];

        p1.execute(&input, &mut regs);

        let regs_res: Vec<i32> = vec![7, 7, 8, 9, 10];

        // Check the len  of the register
        assert_eq!(
            regs.len(),
            regs_res.len(),
            "The size of the register is not correct after execution of the program"
        );

        // Check if the register give the correct result after execution
        assert_eq!(
            regs, regs_res,
            "The register is not correct after execution of the program"
        );
    }

    #[test]
    fn test_mutate_program() {
        let mut init_params: ProgramParams = ProgramParams {
            id_counter_program: 47,
            max_program_length: 5,
            nb_operations: 5,
            input_size: 3,
            nb_destinations: 3,
            p_inst_del: 0.0,
            p_inst_mut: 0.0,
            p_inst_swap: 0.0,
            p_inst_add: 0.0,
        };

        // Define program to test the mutation 'delete'
        let mut p1: Program = Program::new(None, &mut init_params);
        // Define program to test the mutation 'mutate'
        let mut p2: Program = Program::new(None, &mut init_params);
        // Define program to test the mutation 'swap'
        let mut p3: Program = Program::new(None, &mut init_params);
        // Define program to test the mutation 'add'
        let mut p4: Program = Program::new(None, &mut init_params);

        // regs[0] = input[0] + regs[0]
        let i1: Instruction = Instruction::new(1, 0, 0, 0);

        // regs[0] = regs[0] + regs[0]
        let i2: Instruction = Instruction::new(0, 0, 0, 0);

        p1.reset();
        p1.set_instructions(i1.clone());
        p1.set_instructions(i2.clone());

        p2.reset();
        p2.set_instructions(i1.clone());
        p2.set_instructions(i2.clone());

        p3.reset();
        p3.set_instructions(i1.clone());
        p3.set_instructions(i2.clone());

        p4.reset();
        p4.set_instructions(i1.clone());
        p4.set_instructions(i2.clone());

        let mutate_params1: ProgramParams = ProgramParams {
            id_counter_program: 47,
            max_program_length: 5,
            nb_operations: 5,
            input_size: 3,
            nb_destinations: 3,
            p_inst_del: 0.25,
            p_inst_mut: 0.0,
            p_inst_swap: 0.0,
            p_inst_add: 0.0,
        };
        let mutate_params2: ProgramParams = ProgramParams {
            id_counter_program: 47,
            max_program_length: 5,
            nb_operations: 5,
            input_size: 3,
            nb_destinations: 3,
            p_inst_del: 0.0,
            p_inst_mut: 0.25,
            p_inst_swap: 0.0,
            p_inst_add: 0.0,
        };
        let mutate_params3: ProgramParams = ProgramParams {
            id_counter_program: 47,
            max_program_length: 5,
            nb_operations: 5,
            input_size: 3,
            nb_destinations: 3,
            p_inst_del: 0.0,
            p_inst_mut: 0.0,
            p_inst_swap: 0.25,
            p_inst_add: 0.0,
        };
        let mutate_params4: ProgramParams = ProgramParams {
            id_counter_program: 47,
            max_program_length: 5,
            nb_operations: 5,
            input_size: 3,
            nb_destinations: 3,
            p_inst_del: 0.0,
            p_inst_mut: 0.0,
            p_inst_swap: 0.0,
            p_inst_add: 0.25,
        };

        let len_before_del = p1.get_len_program();
        let len_before_mut = p2.get_len_program();
        let len_before_swap = p3.get_len_program();
        let len_before_add = p4.get_len_program();

        // Unit test for the mutation 'delete'
        p1.mutate(&mutate_params1);
        assert_eq!(
            p1.get_len_program(),
            len_before_del - 1,
            "The len of the program is not correct after the mutation 'delete'"
        );

        // Unit test for the mutation 'mutate'
        p2.mutate(&mutate_params2);
        assert_eq!(
            p2.get_len_program(),
            len_before_mut,
            "The len of the program is not correct after the mutation 'mutate'"
        );

        // Unit test for the mutation 'swap'
        p3.mutate(&mutate_params3);
        assert_eq!(
            p3.get_len_program(),
            len_before_swap,
            "The len of the program is not correct after the mutation 'swap'"
        );

        // Unit test for the mutation 'add'
        p4.mutate(&mutate_params4);
        assert_eq!(
            p4.get_len_program(),
            len_before_add + 1,
            "The len of the program is not correct after the mutation 'add'"
        );
    }
}
