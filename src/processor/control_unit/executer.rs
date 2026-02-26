use std::fs::read_to_string;

use crate::processor::{
    control_unit::{
        instruction::{
            BType, BTypeInstruction, Instruction, LoadFileInstruction, LoadImmediateInstruction,
            LoadStringInstruction, MoveInstruction, OutputInstruction, RTypeInstruction,
        },
        language_logic_unit::LanguageLogicUnit,
    },
    memory::Memory,
    registers::{Registers, Value},
};

pub struct Executer {
    language_logic: LanguageLogicUnit,
}

impl Executer {
    pub fn new() -> Self {
        Executer {
            language_logic: LanguageLogicUnit::new(),
        }
    }

    fn load_string(registers: &mut Registers, instruction: &LoadStringInstruction, debug: bool) {
        let value = Value::Text(instruction.value.clone());
       
        registers
            .set_register(instruction.destination_register, &value)
            .unwrap_or_else(|e| panic!("Failed to set register: {}", e));

        if debug {
            println!(
                "Executed LI: r{} = {:?}",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn load_immediate(
        registers: &mut Registers,
        instruction: &LoadImmediateInstruction,
        debug: bool,
    ) {
        let value = Value::Number(instruction.value);
       
        registers
            .set_register(instruction.destination_register, &value)
            .unwrap_or_else(|error| panic!("Failed to set register: {}", error));

        if debug {
            println!(
                "Executed LI: r{} = {:?}",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn load_file(registers: &mut Registers, instruction: &LoadFileInstruction, debug: bool) {
        let file_contents = read_to_string(&instruction.file_path)
            .unwrap_or_else(|error| panic!("Run failed. Error: {}", error));

        registers
            .set_register(
                instruction.destination_register,
                &Value::Text(file_contents),
            )
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to set register for LF instruction. Error: {}",
                    error
                )
            });

        if debug {
            println!(
                "Executed LF: r{} = \"{:?}\"",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn mov(registers: &mut Registers, instruction: &MoveInstruction, debug: bool) {
        let value = registers
            .get_register(instruction.source_register)
            .unwrap_or_else(|error| panic!("Failed to execute MOV instruction. Error: {}", error))
            .to_owned();

        registers
            .set_register(instruction.destination_register, &value)
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to set register for MOV instruction. Error: {}",
                    error
                )
            });

        if debug {
            println!(
                "Executed MOV: r{} = \"{:?}\"",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn r_type(&mut self, registers: &mut Registers, instruction: &RTypeInstruction, debug: bool) {
        // read two text operands
        let value_a = registers
            .read_text(instruction.source_register_1)
            .unwrap_or_else(|error| panic!("{}", error));
        let value_b = registers
            .read_text(instruction.source_register_2)
            .unwrap_or_else(|error| panic!("{}", error));

        let result = self
            .language_logic
            .run(&instruction.r_type, value_a, value_b)
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to perform {:?}. Error: {}",
                    instruction.r_type, error
                )
            });

        if debug {
            println!(
                "Executed {:?}: '{}' , '{}', -> r{} = '{:?}'",
                instruction.r_type, value_a, value_b, instruction.destination_register, result
            );
        }

        registers
            .set_register(instruction.destination_register, &result)
            .unwrap_or_else(|error| panic!("Failed to set register: {}", error));
    }

    fn b_type(registers: &mut Registers, instruction: &BTypeInstruction, debug: bool) {
        let value_a = registers
            .read_number(instruction.source_register_1)
            .unwrap_or_else(|error| panic!("{}", error));

        let value_b = registers
            .read_number(instruction.source_register_2)
            .unwrap_or_else(|error| panic!("{}", error));

        let is_true = match instruction.b_type {
            BType::Equal => value_a == value_b,
            BType::Less => value_a < value_b,
            BType::LessEqual => value_a <= value_b,
            BType::Greater => value_a > value_b,
            BType::GreaterEqual => value_a >= value_b,
        };

        if is_true {
            registers.set_instruction_pointer(
                usize::try_from(instruction.instruction_pointer_jump_index).unwrap_or_else(
                    |error| {
                        panic!(
                            "Failed to convert instruction pointer jump index to usize. Error: {}",
                            error
                        )
                    },
                ),
            );
        }

        if debug {
            println!(
                "Executed {:?}: {:?} {:?} -> {} jump {}",
                instruction.b_type,
                value_a,
                value_b,
                is_true,
                instruction.instruction_pointer_jump_index
            );
        }
    }

    fn output(registers: &Registers, instruction: &OutputInstruction, debug: bool) {
        let value = registers
            .get_register(instruction.source_register)
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to read register r{}: {}",
                    instruction.source_register, error
                )
            });
        let output = value.to_string();

        if debug {
            println!("Executed OUT: {}", output);
        } else {
            println!("{}", output);
        }
    }

    fn exit(memory: &Memory, registers: &mut Registers, debug: bool) {
        if debug {
            println!("Executed EXIT: Halting execution.");
        }

        registers.set_instruction_pointer(memory.length());
    }

    pub fn execute(
        &mut self,
        memory: &mut Memory,
        registers: &mut Registers,
        instruction: &Instruction,
        debug: bool,
    ) {
        match instruction {
            Instruction::LoadString(i) => Self::load_string(registers, i, debug),
            Instruction::LoadImmediate(i) => Self::load_immediate(registers, i, debug),
            Instruction::LoadFile(i) => Self::load_file(registers, i, debug),
            Instruction::Move(i) => Self::mov(registers, i, debug),
            Instruction::BType(i) => Self::b_type(registers, i, debug),
            Instruction::Exit(_i) => Self::exit(memory, registers, debug),
            Instruction::Output(i) => Self::output(registers, i, debug),
            Instruction::RType(i) => Self::r_type(self, registers, i, debug),
        }
    }
}
