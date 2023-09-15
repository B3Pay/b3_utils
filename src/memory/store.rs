use std::cell::RefCell;

use super::{backup::BackupPartition, error::StableMemoryError, StableMemory};

thread_local! {
    pub static STABLE_MEMORY: RefCell<StableMemory> = RefCell::new(StableMemory::init())
}

/// return the refcell of the new stable memory
pub fn add_and_use_memory<F, R>(name: &str, id: u8, f: F) -> Result<R, StableMemoryError>
where
    F: FnOnce(&super::types::DefaultVM) -> R,
{
    with_stable_memory_mut(|pm| {
        // Create a new memory partition with the given name and id
        match pm.create(name, id) {
            Ok(memory) => {
                // Use the newly created memory partition
                Ok(f(&memory))
            }
            Err(e) => Err(e),
        }
    })
}

pub fn with_stable_memory<F, R>(f: F) -> R
where
    F: FnOnce(&StableMemory) -> R,
{
    STABLE_MEMORY.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_stable_memory_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut StableMemory) -> R,
{
    STABLE_MEMORY.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}

pub fn with_backup_memory<F, R>(f: F) -> R
where
    F: FnOnce(&BackupPartition) -> R,
{
    with_stable_memory(|pm| {
        let bp = pm.backup();
        f(&bp)
    })
}

pub fn with_backup_memory_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut BackupPartition) -> R,
{
    with_stable_memory_mut(|pm| {
        let mut bp = pm.backup_mut();
        f(&mut bp)
    })
}

pub fn with_partition<F, R>(name: &str, f: F) -> R
where
    F: FnOnce(&super::types::DefaultVM) -> R,
{
    with_stable_memory(|pm| {
        let memory = pm
            .memory(name)
            .expect(&format!("Unable to find memory with name: {}", name));
        f(&memory)
    })
}
