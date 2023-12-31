use std::{borrow::Cow, cmp::Ordering};

use candid::CandidType;
use ic_stable_structures::{storable::Bound, vec::InitError, GrowFailed, Storable};
use serde::{Deserialize, Serialize};

use crate::{memory::DefaultStableMinHeap, NanoTimeStamp};

use super::types::DefaultVM;

// Added the missing generic type parameter <T>
pub struct DefaultTaskTimer<T: Storable>(DefaultStableMinHeap<TaskTimerEntry<T>>);

// Added the missing generic type parameter <T>  trait bound
impl<T: Storable> DefaultTaskTimer<T> {
    pub fn init(vm: DefaultVM) -> Result<Self, InitError> {
        let task_timer = Self(DefaultStableMinHeap::init(vm)?);

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

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct TaskTimerEntry<T> {
    pub time: NanoTimeStamp,
    pub task: T,
}

impl<T> PartialOrd for TaskTimerEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for TaskTimerEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl<T> PartialEq for TaskTimerEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<T> Eq for TaskTimerEntry<T> {}

impl<T: Storable> Storable for TaskTimerEntry<T> {
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

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: false,
        max_size: 8 + T::BOUND.max_size(),
    };
}
