use std::sync::{Arc, Mutex, MutexGuard};

use crate::processor::{memory::Memory, registers::Registers};

/// Convenience function for locking the shared memory arc.
///
/// This centralizes the error message used throughout the control unit
/// so the various components don't each need to duplicate it.
pub fn memory_lock(memory: &Arc<Mutex<Memory>>) -> MutexGuard<'_, Memory> {
    memory
        .lock()
        .expect("Failed to access memory: memory lock error")
}

/// Convenience function for locking the shared registers arc.
///
/// See [`memory_lock`] for reasoning; the implementation is identical
/// apart from the type and error message.
pub fn registers_lock(
    registers: &Arc<Mutex<Registers>>,
) -> MutexGuard<'_, Registers> {
    registers
        .lock()
        .expect("Failed to access registers: registers lock error")
}
