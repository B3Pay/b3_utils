use std::cell::RefCell;

use super::BackupPartition;

thread_local! {
    pub static BACKUP_PARTITION: RefCell<BackupPartition> = todo!("init backup partition");
    // static BACKUP_PARTITION: RefCell<BackupPartition> = RefCell::new(with_stable_memory_mut(|pm| BackupPartition::init(pm)));
}

pub fn with_backup_partition<F, R>(f: F) -> R
where
    F: FnOnce(&BackupPartition) -> R,
{
    BACKUP_PARTITION.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_backup_partition_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut BackupPartition) -> R,
{
    BACKUP_PARTITION.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}
