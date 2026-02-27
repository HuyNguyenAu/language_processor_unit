use std::fs::read_to_string;

use crate::processor::{
    control_unit::{
        instruction::{
            AuditInstruction, BranchInstruction, BranchType, ContextPopInstruction,
            ContextPushInstruction, ContextRestoreInstruction, ContextSetRoleInstruction,
            ContextSnapshotInstruction, CorrelateInstruction, DistillInstruction, Instruction,
            LoadFileInstruction, LoadImmediateInstruction, LoadStringInstruction, MorphInstruction,
            MoveInstruction, OutputInstruction, ProjectInstruction, SimilarityInstruction,
        },
        language_logic_unit::LanguageLogicUnit,
        roles,
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
            "Executed LF: r{} = {:?}",
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
            "Executed MOV: r{} = {:?}",
            instruction.destination_register,
            registers.get_register(instruction.destination_register)
        );
    }

    fn branch(registers: &mut Registers, instruction: &BranchInstruction, debug: bool) {
        let value_a = Self::read_number(registers, instruction.source_register_1)
            .expect("Failed to read number from register");

        let value_b = Self::read_number(registers, instruction.source_register_2)
            .expect("Failed to read number from register");

        let is_true = match instruction.branch_type {
            BranchType::Equal => value_a == value_b,
            BranchType::Less => value_a < value_b,
            BranchType::LessEqual => value_a <= value_b,
            BranchType::Greater => value_a > value_b,
            BranchType::GreaterEqual => value_a >= value_b,
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
            instruction.branch_type,
            value_a,
            value_b,
            is_true,
            instruction.instruction_pointer_jump_index
        );
    }

    fn morph(registers: &mut Registers, instruction: &MorphInstruction, debug: bool) {
        let value = Self::read_text(registers, instruction.source_register)
            .expect("Failed to read text from source register for MORPH instruction");
        let micro_prompt = format!(
            "Transform it into the following format:\n{}\n\nTransformed Output:",
            value
        );
        let context = registers.get_context().clone();

        let result = LanguageLogicUnit::string(&micro_prompt, context)
            .unwrap_or_else(|error| panic!("Failed to perform MORPH operation. Error: {}", error));

        crate::debug_print!(
            debug,
            "Executed MORPH: r{} = '{:?}'",
            instruction.destination_register,
            result
        );

        registers
            .set_register(instruction.destination_register, &Value::Text(result))
            .expect("Failed to set register");
    }

    fn project(registers: &mut Registers, instruction: &ProjectInstruction, debug: bool) {
        let value = Self::read_text(registers, instruction.source_register)
            .expect("Failed to read text from source register for PROJECT instruction");
        let micro_prompt = format!(
            "Project how it might evolve based on this direction or trend:\n{}\n\nProjected Output:",
            value
        );
        let context = registers.get_context().clone();

        let result = LanguageLogicUnit::string(&micro_prompt, context).unwrap_or_else(|error| {
            panic!("Failed to perform PROJECT operation. Error: {}", error)
        });

        crate::debug_print!(
            debug,
            "Executed PROJECT: r{} = '{:?}'",
            instruction.destination_register,
            result
        );

        registers
            .set_register(instruction.destination_register, &Value::Text(result))
            .expect("Failed to set register");
    }

    fn distill(registers: &mut Registers, instruction: &DistillInstruction, debug: bool) {
        let value = Self::read_text(registers, instruction.source_register)
            .expect("Failed to read text from source register for DISTILL instruction");
        let micro_prompt = format!(
            "Distill it down following the goal or criteria:\n{}\n\nDistilled Result:",
            value
        );
        let context = registers.get_context().clone();

        let result = LanguageLogicUnit::string(&micro_prompt, context).unwrap_or_else(|error| {
            panic!("Failed to perform DISTILL operation. Error: {}", error)
        });

        crate::debug_print!(
            debug,
            "Executed DISTILL: r{} = '{:?}'",
            instruction.destination_register,
            result
        );

        registers
            .set_register(instruction.destination_register, &Value::Text(result))
            .expect("Failed to set register");
    }

    fn correlate(registers: &mut Registers, instruction: &CorrelateInstruction, debug: bool) {
        let value = Self::read_text(registers, instruction.source_register)
            .expect("Failed to read text from source register for CORRELATE instruction");
        let micro_prompt = format!(
            "Find the correlation with:\n{}\n\nRelational Analysis:",
            value
        );
        let context = registers.get_context().clone();

        let result = LanguageLogicUnit::string(&micro_prompt, context).unwrap_or_else(|error| {
            panic!("Failed to perform CORRELATE operation. Error: {}", error)
        });

        crate::debug_print!(
            debug,
            "Executed CORRELATE: r{} = '{:?}'",
            instruction.destination_register,
            result
        );

        registers
            .set_register(instruction.destination_register, &Value::Text(result))
            .expect("Failed to set register");
    }

    fn audit(registers: &mut Registers, instruction: &AuditInstruction, debug: bool) {
        let value = Self::read_text(registers, instruction.source_register)
            .expect("Failed to read text from source register for AUDIT instruction");
        let micro_prompt = format!("Does it comply with:\n{}\n\nYES/NO:", value);
        let true_values = vec!["YES"];
        let false_values = vec!["NO"];
        let context = registers.get_context().clone();

        let result = LanguageLogicUnit::boolean(&micro_prompt, true_values, false_values, context)
            .unwrap_or_else(|error| panic!("Failed to perform AUDIT operation. Error: {}", error));

        crate::debug_print!(
            debug,
            "Executed AUDIT: r{} = '{:?}'",
            instruction.destination_register,
            result
        );

        registers
            .set_register(instruction.destination_register, &Value::Number(result))
            .expect("Failed to set register");
    }

    fn similarity(registers: &mut Registers, instruction: &SimilarityInstruction, debug: bool) {
        let value_a = Self::read_text(registers, instruction.source_register_1)
            .expect("Failed to read text from source register 1 for SIMILARITY instruction");
        let value_b = Self::read_text(registers, instruction.source_register_2)
            .expect("Failed to read text from source register 2 for SIMILARITY instruction");

        let result =
            LanguageLogicUnit::cosine_similarity(&value_a, &value_b).unwrap_or_else(|error| {
                panic!("Failed to perform SIMILARITY operation. Error: {}", error)
            });

        crate::debug_print!(
            debug,
            "Executed SIMILARITY: '{:?}' vs '{:?}' -> r{} = {}",
            value_a,
            value_b,
            instruction.destination_register,
            result
        );

        registers
            .set_register(instruction.destination_register, &Value::Number(result))
            .expect("Failed to set register");
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
        let role = registers
            .get_context_role()
            .unwrap_or(roles::USER_ROLE.to_string())
            .to_string();

        registers.push_context(ContextMessage::new(&role, &value));

        crate::debug_print!(
            debug && registers.get_context_role().is_none(),
            "Defaulting context role to '{}' for CONTEXT_PUSH since no role is currently set.",
            roles::USER_ROLE
        );
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

    fn context_set_role(
        registers: &mut Registers,
        instruction: &ContextSetRoleInstruction,
        debug: bool,
    ) {
        registers.set_context_role(&instruction.role);

        crate::debug_print!(
            debug,
            "Executed SRL: Set context role to '{}'.",
            instruction.role
        );
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
            // Data movement operations.
            Instruction::LoadString(i) => Self::load_string(registers, i, debug),
            Instruction::LoadImmediate(i) => Self::load_immediate(registers, i, debug),
            Instruction::LoadFile(i) => Self::load_file(registers, i, debug),
            Instruction::Move(i) => Self::mov(registers, i, debug),
            // Control flow operations.
            Instruction::Branch(i) => Self::branch(registers, i, debug),
            Instruction::Exit(_) => Self::exit(memory, registers, debug),
            // Generative operations.
            Instruction::Morph(i) => Self::morph(registers, i, debug),
            Instruction::Project(i) => Self::project(registers, i, debug),
            // Cognitive operations.
            Instruction::Distill(i) => Self::distill(registers, i, debug),
            Instruction::Correlate(i) => Self::correlate(registers, i, debug),
            // Guardrails operations.
            Instruction::Audit(i) => Self::audit(registers, i, debug),
            Instruction::Similarity(i) => Self::similarity(registers, i, debug),
            // Context operations.
            Instruction::ContextClear(_) => Self::context_clear(registers, debug),
            Instruction::ContextSnapshot(i) => Self::context_snapshot(registers, i, debug),
            Instruction::ContextRestore(i) => Self::context_restore(registers, i, debug),
            Instruction::ContextPush(i) => Self::context_push(registers, i, debug),
            Instruction::ContextPop(i) => Self::context_pop(registers, i, debug),
            Instruction::ContextDrop(_) => Self::context_drop(registers, debug),
            Instruction::ContextSetRole(i) => Self::context_set_role(registers, i, debug),
            // I/O operations.
            Instruction::Output(i) => Self::output(registers, i, debug),
        }
    }
}
