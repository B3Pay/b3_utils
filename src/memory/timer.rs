use std::borrow::Cow;

use b3_stable_structures::{vec::InitError, BoundedStorable, GrowFailed, Storable};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{memory::DefaultVMHeap, NanoTimeStamp};

use super::types::DefaultVM;

// Added the missing generic type parameter <T>
pub struct DefaultTaskTimer<T: Ord + PartialOrd + Storable + BoundedStorable>(
    DefaultVMHeap<TaskTimerEntry<T>>,
);

#[derive(CandidType, Debug, PartialEq, Eq, Ord, Clone, Serialize, Deserialize)]
pub struct TaskTimerEntry<T> {
    pub time: NanoTimeStamp,
    pub task: T,
}

// Added the missing generic type parameter <T> and the BoundedStorable trait bound
impl<T: Ord + PartialOrd + Storable + BoundedStorable> DefaultTaskTimer<T> {
    pub fn init(vm: DefaultVM) -> Result<Self, InitError> {
        let task_timer = Self(DefaultVMHeap::init(vm)?);

        Ok(task_timer)
    }

    pub fn peek_timer(&self) -> Option<TaskTimerEntry<T>> {
        self.0.peek()
    }

    pub fn timers(&self) -> &DefaultTaskTimer<T> {
        &self
    }

    pub fn get_timers(&self) -> Vec<TaskTimerEntry<T>> {
        self.0.iter().collect()
    }

    pub fn timers_mut(&mut self) -> &mut DefaultTaskTimer<T> {
        self
    }

    pub fn push_timer(&mut self, timer: &TaskTimerEntry<T>) -> Result<(), GrowFailed> {
        self.0.push(timer)
    }

    pub fn pop_timer(&mut self) -> Option<TaskTimerEntry<T>> {
        self.0.pop()
    }

    pub fn clear_timer(&mut self) {
        for _ in 0..self.0.len() {
            self.0.pop();
        }
    }
}

impl<T> PartialOrd for TaskTimerEntry<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl<T: Ord + PartialOrd + Storable + BoundedStorable> BoundedStorable for TaskTimerEntry<T> {
    const IS_FIXED_SIZE: bool = false;
    const MAX_SIZE: u32 = 8 + T::MAX_SIZE;
}

impl<T: Ord + PartialOrd + Storable + BoundedStorable> Storable for TaskTimerEntry<T> {
    fn to_bytes(&self) -> Cow<[u8]> {
        let time_bytes = self.time.to_le_bytes();
        let task_bytes = self.task.to_bytes();

        // Now the total size is dynamic based on the size of task_bytes
        let total_size = 8 + task_bytes.len();
        let mut bytes = vec![0; total_size];

        bytes[0..8].copy_from_slice(&time_bytes);
        bytes[8..8 + task_bytes.len()].copy_from_slice(&task_bytes);

        bytes.into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let time = NanoTimeStamp::from_le_bytes(bytes[0..8].try_into().unwrap());

        // Use the rest of the bytes for the task
        let task = T::from_bytes(bytes[8..].into());

        Self { time, task }
    }
}
