use std::{
    fs::read_to_string,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::processor::{
    control_unit::{
        instruction::{
            BType, BTypeInstruction, ExitInstruction, Instruction, LoadFileInstruction,
            LoadImmediateInstruction, LoadStringInstruction, MoveInstruction, OutputInstruction,
            RTypeInstruction,
        },
        language_logic_unit::LanguageLogicUnit,
    },
    memory::Memory,
    registers::{Registers, Value},
};

pub struct Executer {
    memory: Arc<Mutex<Memory>>,
    registers: Arc<Mutex<Registers>>,
    language_logic_unit: LanguageLogicUnit,
}

impl Executer {
    pub fn new(memory: &Arc<Mutex<Memory>>, registers: &Arc<Mutex<Registers>>) -> Self {
        Executer {
            memory: Arc::clone(memory),
            registers: Arc::clone(registers),
            language_logic_unit: LanguageLogicUnit::new(),
        }
    }

    fn memory_lock(&self) -> MutexGuard<'_, Memory> {
        match self.memory.lock() {
            Ok(memory) => memory,
            Err(error) => {
                panic!("Failed to access memory: memory lock error: {}", error);
            }
        }
    }

    fn registers_lock(&self) -> MutexGuard<'_, Registers> {
        match self.registers.lock() {
            Ok(registers) => registers,
            Err(error) => {
                panic!(
                    "Failed to access registers: registers lock error: {}",
                    error
                );
            }
        }
    }

    fn load_string(&mut self, instruction: &LoadStringInstruction, debug: bool) {
        let mut registers = self.registers_lock();

        match registers.set_register(
            instruction.destination_register,
            &Value::Text(instruction.value.clone()),
        ) {
            Ok(_) => (),
            Err(error) => panic!(
                "Failed to set register for LI instruction. Error: {}",
                error
            ),
        };

        if debug {
            println!(
                "Executed LI: r{} = \"{:?}\"",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn load_immediate(&mut self, instruction: &LoadImmediateInstruction, debug: bool) {
        let mut registers = self.registers_lock();

        match registers.set_register(
            instruction.destination_register,
            &Value::Number(instruction.value),
        ) {
            Ok(_) => (),
            Err(error) => panic!(
                "Failed to set register for LI instruction. Error: {}",
                error
            ),
        };

        if debug {
            println!(
                "Executed LI: r{} = \"{:?}\"",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn load_file(&mut self, instruction: &LoadFileInstruction, debug: bool) {
        let mut registers = self.registers_lock();

        let file_contents = match read_to_string(&instruction.file_path) {
            Ok(value) => value,
            Err(error) => panic!("Run failed. Error: {}", error),
        };

        match registers.set_register(
            instruction.destination_register,
            &Value::Text(file_contents),
        ) {
            Ok(_) => (),
            Err(error) => panic!(
                "Failed to set register for LF instruction. Error: {}",
                error
            ),
        };

        if debug {
            println!(
                "Executed LF: r{} = \"{:?}\"",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn _move(&mut self, instruction: &MoveInstruction, debug: bool) {
        let mut registers = self.registers_lock();

        let value = match registers.get_register(instruction.source_register) {
            Ok(value) => value.to_owned(),
            Err(error) => panic!("Failed to execute MOV instruction. Error: {}", error),
        };

        match registers.set_register(instruction.destination_register, &value) {
            Ok(_) => (),
            Err(error) => panic!(
                "Failed to set register for MOV instruction. Error: {}",
                error
            ),
        };

        if debug {
            println!(
                "Executed MOV: r{} = \"{:?}\"",
                instruction.destination_register,
                registers.get_register(instruction.destination_register)
            );
        }
    }

    fn r_type(&mut self, instruction: &RTypeInstruction, debug: bool) {
        let mut registers = self.registers_lock();

        let value_a = match registers.get_register(instruction.source_register_1) {
            Ok(value) => value,
            Err(error) => panic!(
                "Failed to read source register r{} for R-type instruction. Error: {}",
                instruction.source_register_1, error
            ),
        };

        let value_b = match registers.get_register(instruction.source_register_2) {
            Ok(value) => value,
            Err(error) => panic!(
                "Failed to read source register r{} for R-type instruction. Error: {}",
                instruction.source_register_2, error
            ),
        };

        let result = match self
            .language_logic_unit
            .run(&instruction.r_type, value_a, value_b)
        {
            Ok(result) => result,
            Err(error) => panic!(
                "Failed to perform {:?}. Error: {}",
                instruction.r_type, error
            ),
        };

        if debug {
            println!(
                "Executed {:?}: {:?} {:?} {:?} -> r{} = \"{:?}\"",
                instruction.r_type,
                value_a,
                instruction.r_type,
                value_b,
                instruction.destination_register,
                result
            );
        }

        match registers.set_register(instruction.destination_register, &result) {
            Ok(_) => {}
            Err(error) => panic!(
                "Failed to set register for {:?} instruction. Error: {}",
                instruction.r_type, error
            ),
        };
    }

    fn b_type(&mut self, instruction: &BTypeInstruction, debug: bool) {
        let mut registers = self.registers_lock();

        let value_a = match registers.get_register(instruction.source_register_1) {
            Ok(Value::Number(value)) => *value,
            Ok(_) => panic!(
                "Expected numeric value in source register r{} for B-type instruction.",
                instruction.source_register_1
            ),
            Err(error) => panic!(
                "Failed to read source register r{} for B-type instruction. Error: {}",
                instruction.source_register_1, error
            ),
        };

        let value_b = match registers.get_register(instruction.source_register_2) {
            Ok(Value::Number(value)) => *value,
            Ok(_) => panic!(
                "Expected numeric value in source register r{} for B-type instruction.",
                instruction.source_register_2
            ),
            Err(error) => panic!(
                "Failed to read source register r{} for B-type instruction. Error: {}",
                instruction.source_register_2, error
            ),
        };

        let instruction_pointer = instruction.instruction_pointer_jump_index;
        let is_true = match instruction.b_type {
            BType::Equal => value_a == value_b,
            BType::Less => value_a < value_b,
            BType::LessEqual => value_a <= value_b,
            BType::Greater => value_a > value_b,
            BType::GreaterEqual => value_a >= value_b,
        };

        if is_true {
            let address = match usize::try_from(instruction_pointer) {
                Ok(address) => address,
                Err(_) => panic!(
                    "Failed to convert address to usize for branch instruction. Address value: {}. Address value must be between 0 and {}.",
                    instruction_pointer,
                    usize::MAX
                ),
            };

            registers.set_instruction_pointer(address);
        }

        if debug {
            match instruction.b_type {
                BType::Equal => {
                    println!(
                        "Executed {:?}: {:?} == {:?} -> {}, {}",
                        instruction.b_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.instruction_pointer_jump_index
                    );
                }
                BType::Less => {
                    println!(
                        "Executed {:?}: {:?} < {:?} -> {}, {}",
                        instruction.b_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.instruction_pointer_jump_index
                    );
                }
                BType::LessEqual => {
                    println!(
                        "Executed {:?}: {:?} <= {:?} -> {}, {}",
                        instruction.b_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.instruction_pointer_jump_index
                    );
                }
                BType::Greater => {
                    println!(
                        "Executed {:?}: {:?} > {:?} -> {}, {}",
                        instruction.b_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.instruction_pointer_jump_index
                    );
                }
                BType::GreaterEqual => println!(
                    "Executed {:?}: {:?} >= {:?} -> {}, {}",
                    instruction.b_type,
                    value_a,
                    value_b,
                    is_true,
                    instruction.instruction_pointer_jump_index
                ),
            }
        }
    }

    fn output(&mut self, instruction: &OutputInstruction, debug: bool) {
        let registers = self.registers_lock();

        let value_a = match registers.get_register(instruction.source_register) {
            Ok(Value::Text(value)) => value.clone(),
            Ok(Value::Number(value)) => value.to_string(),
            Ok(_) => panic!(
                "Expected text or numeric value in source register r{} for OUT instruction.",
                instruction.source_register
            ),
            Err(error) => panic!(
                "Failed to read source register r{} for OUT instruction. Error: {}",
                instruction.source_register, error
            ),
        };

        if debug {
            println!("Executed OUT: {}", value_a);
        } else {
            println!("{}", value_a);
        }
    }

    fn exit(&mut self, _instruction: &ExitInstruction, debug: bool) {
        let memory = self.memory_lock();
        let mut registers = self.registers_lock();

        if debug {
            println!("Executed EXIT: Halting execution.");
        }

        // Set instruction pointer to memory length to indicate end of execution.
        registers.set_instruction_pointer(memory.length());
    }

    pub fn execute(&mut self, instruction: &Instruction, debug: bool) {
        match instruction {
            Instruction::LoadString(instruction) => self.load_string(instruction, debug),
            Instruction::LoadImmediate(instruction) => self.load_immediate(instruction, debug),
            Instruction::LoadFile(instruction) => self.load_file(instruction, debug),
            Instruction::Move(instruction) => self._move(instruction, debug),
            Instruction::BType(instruction) => self.b_type(instruction, debug),
            Instruction::Exit(instruction) => self.exit(instruction, debug),
            Instruction::Output(instruction) => self.output(instruction, debug),
            Instruction::RType(instruction) => self.r_type(instruction, debug),
        }
    }
}
