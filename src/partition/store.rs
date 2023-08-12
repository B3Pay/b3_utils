use std::cell::RefCell;

use super::Partition;

thread_local! {
    pub static PARTITION: RefCell<Partition> = RefCell::new(Partition::init())
}

pub fn with_partition<F, R>(f: F) -> R
where
    F: FnOnce(&Partition) -> R,
{
    PARTITION.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_partition_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut Partition) -> R,
{
    PARTITION.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}
