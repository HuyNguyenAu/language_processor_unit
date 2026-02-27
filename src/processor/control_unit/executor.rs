use std::fs::read_to_string;

use crate::processor::{
    control_unit::{
        instruction::{
            BType, BTypeInstruction, ContextPopInstruction, ContextPushInstruction,
            ContextRestoreInstruction, ContextSnapshotInstruction, Instruction,
            LoadFileInstruction, LoadImmediateInstruction, LoadStringInstruction, MoveInstruction,
            OutputInstruction, RType, RTypeInstruction,
        },
        language_logic_unit::{LanguageLogicUnit, USER_ROLE},
    },
    memory::Memory,
    registers::{ContextMessage, Registers, Value},
};

pub struct Executor;

impl Executor {
    fn read_text(registers: &Registers, register_number: u32) -> Result<&String, String> {
        match registers.get_register(register_number)? {
            Value::Text(text) => Ok(text),
            other => Err(format!(
                "Register r{} contains {:?}, expected text.",
                register_number, other
            )),
        }
    }

    fn read_number(registers: &Registers, register_number: u32) -> Result<u32, String> {
        match registers.get_register(register_number)? {
            Value::Number(number) => Ok(*number),
            other => Err(format!(
                "Register r{} contains {:?}, expected number.",
                register_number, other
            )),
        }
    }

    fn load_string(registers: &mut Registers, instruction: &LoadStringInstruction, debug: bool) {
        let value = Value::Text(instruction.value.clone());

        registers
            .set_register(instruction.destination_register, &value)
            .expect("Failed to set register");

        crate::debug_print!(
            debug,
            "Executed LI: r{} = {:?}",
            instruction.destination_register,
            value
        );
    }

    fn load_immediate(
        registers: &mut Registers,
        instruction: &LoadImmediateInstruction,
        debug: bool,
    ) {
        let value = Value::Number(instruction.value);

        registers
            .set_register(instruction.destination_register, &value)
            .expect("Failed to set register");

        crate::debug_print!(
            debug,
            "Executed LI: r{} = {:?}",
            instruction.destination_register,
            value
        );
    }

    fn load_file(registers: &mut Registers, instruction: &LoadFileInstruction, debug: bool) {
        let file_contents =
            read_to_string(&instruction.file_path).expect("Run failed while reading file");

        registers
            .set_register(
                instruction.destination_register,
                &Value::Text(file_contents.clone()),
            )
            .expect("Failed to set register for LF instruction");

        crate::debug_print!(
            debug,
            "Executed LF: r{} = \"{:?}\"",
            instruction.destination_register,
            file_contents
        );
    }

    fn mov(registers: &mut Registers, instruction: &MoveInstruction, debug: bool) {
        let value = registers
            .get_register(instruction.source_register)
            .expect("Failed to execute MOV instruction")
            .clone();

        registers
            .set_register(instruction.destination_register, &value)
            .expect("Failed to set register for MOV instruction");

        crate::debug_print!(
            debug,
            "Executed MOV: r{} = \"{:?}\"",
            instruction.destination_register,
            registers.get_register(instruction.destination_register)
        );
    }

    fn r_type(registers: &mut Registers, instruction: &RTypeInstruction, debug: bool) {
        let value_a = Self::read_text(registers, instruction.source_register_1)
            .expect("Failed to read text from register");
        let value_b = Self::read_text(registers, instruction.source_register_2)
            .expect("Failed to read text from register");
        let context = registers.get_context();

        let result = if matches!(instruction.r_type, RType::Similarity) {
            let value = LanguageLogicUnit::new()
                .boolean(&instruction.r_type, value_a, context.clone())
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to perform {:?}. Error: {}",
                        instruction.r_type, error
                    )
                });

            Value::Number(value)
        } else {
            let value = LanguageLogicUnit::new()
                .string(&instruction.r_type, value_a, context.clone())
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to perform {:?}. Error: {}",
                        instruction.r_type, error
                    )
                });

            Value::Text(value)
        };

        crate::debug_print!(
            debug,
            "Executed {:?}: '{}' , '{}', -> r{} = '{:?}'",
            instruction.r_type,
            value_a,
            value_b,
            instruction.destination_register,
            result
        );

        registers
            .set_register(instruction.destination_register, &result)
            .expect("Failed to set register");
    }

    fn b_type(registers: &mut Registers, instruction: &BTypeInstruction, debug: bool) {
        let value_a = Self::read_number(registers, instruction.source_register_1)
            .expect("Failed to read number from register");

        let value_b = Self::read_number(registers, instruction.source_register_2)
            .expect("Failed to read number from register");

        let is_true = match instruction.b_type {
            BType::Equal => value_a == value_b,
            BType::Less => value_a < value_b,
            BType::LessEqual => value_a <= value_b,
            BType::Greater => value_a > value_b,
            BType::GreaterEqual => value_a >= value_b,
        };

        if is_true {
            registers.set_instruction_pointer(
                usize::try_from(instruction.instruction_pointer_jump_index)
                    .expect("instruction pointer jump index too large"),
            );
        }

        crate::debug_print!(
            debug,
            "Executed {:?}: {:?} {:?} -> {} jump {}",
            instruction.b_type,
            value_a,
            value_b,
            is_true,
            instruction.instruction_pointer_jump_index
        );
    }

    fn context_clear(registers: &mut Registers, debug: bool) {
        registers.clear_context();

        crate::debug_print!(debug, "Executed CLR: Cleared context stack.");
    }

    fn context_snapshot(
        registers: &mut Registers,
        instruction: &ContextSnapshotInstruction,
        debug: bool,
    ) {
        let snapshot = registers.snapshot_context();

        registers
            .set_register(instruction.destination_register, &Value::Text(snapshot))
            .expect("Failed to set register for CONTEXT_SNAPSHOT instruction");

        crate::debug_print!(
            debug,
            "Executed SNP: Snapshotted context stack into r{}.",
            instruction.destination_register
        );
    }

    fn context_restore(
        registers: &mut Registers,
        instruction: &ContextRestoreInstruction,
        debug: bool,
    ) {
        let snapshot = Self::read_text(registers, instruction.source_register)
            .expect("Failed to read context snapshot from register")
            .clone();

        registers
            .restore_context(&snapshot)
            .expect("Failed to restore context from snapshot");

        crate::debug_print!(
            debug,
            "Executed RST: Restored context stack from snapshot in r{}.",
            instruction.source_register
        );
    }

    fn context_push(registers: &mut Registers, instruction: &ContextPushInstruction, debug: bool) {
        let value = match registers
            .get_register(instruction.source_register)
            .expect(&format!(
                "Failed to read register r{} for CONTEXT_PUSH instruction",
                instruction.source_register
            )) {
            Value::Text(text) => text.clone(),
            Value::Number(number) => number.to_string(),
            Value::None => panic!(
                "Register r{} contains None, expected text or number.",
                instruction.source_register
            ),
        };

        registers.push_context(ContextMessage::new(USER_ROLE, &value));

        crate::debug_print!(
            debug,
            "Executed PSH: Pushed value from r{} onto context stack.",
            instruction.source_register
        );
    }

    fn context_pop(registers: &mut Registers, instruction: &ContextPopInstruction, debug: bool) {
        let context = registers
            .pop_context()
            .expect("Failed to pop context because context stack is empty.");

        registers
            .set_register(
                instruction.destination_register,
                &Value::Text(context.content.clone()),
            )
            .expect("Failed to set register for CONTEXT_POP instruction");

        crate::debug_print!(debug, "Executed POP: Popped value from context stack.",);
    }

    fn context_drop(registers: &mut Registers, debug: bool) {
        registers
            .pop_context()
            .expect("Failed to pop context because context stack is empty.");

        crate::debug_print!(debug, "Executed DRP: Dropped value from context stack.",);
    }

    fn output(registers: &Registers, instruction: &OutputInstruction, debug: bool) {
        let value = registers
            .get_register(instruction.source_register)
            .expect(&format!(
                "Failed to read register r{}",
                instruction.source_register
            ))
            .to_string();

        if debug {
            println!("Executed OUT: {}", value);
        } else {
            println!("{}", value);
        }
    }

    fn exit(memory: &Memory, registers: &mut Registers, debug: bool) {
        crate::debug_print!(debug, "Executed EXIT: Halting execution.");
        registers.set_instruction_pointer(memory.length());
    }

    pub fn execute(
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
            Instruction::Exit(_) => Self::exit(memory, registers, debug),
            Instruction::ContextClear(_) => Self::context_clear(registers, debug),
            Instruction::ContextSnapshot(i) => Self::context_snapshot(registers, i, debug),
            Instruction::ContextRestore(i) => Self::context_restore(registers, i, debug),
            Instruction::ContextPush(i) => Self::context_push(registers, i, debug),
            Instruction::ContextPop(i) => Self::context_pop(registers, i, debug),
            Instruction::ContextDrop(_) => Self::context_drop(registers, debug),
            Instruction::Output(i) => Self::output(registers, i, debug),
            Instruction::RType(i) => Self::r_type(registers, i, debug),
        }
    }
}
