use std::borrow::Cow;

use b3_stable_structures::{BoundedStorable, GrowFailed, Storable};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    memory::{DefaultVMHeap, StableMemory},
    NanoTimeStamp,
};

use super::types::PartitionDetail;

// Added the missing generic type parameter <T>
pub struct TaskTimerPartition<T: Ord + PartialOrd + Storable + BoundedStorable>(
    DefaultVMHeap<TaskTimerEntry<T>>,
);

#[derive(CandidType, Debug, PartialEq, Eq, Ord, Clone, Serialize, Deserialize)]
pub struct TaskTimerEntry<T> {
    pub time: NanoTimeStamp,
    pub task: T,
}

// Added the missing generic type parameter <T> and the BoundedStorable trait bound
impl<T: Ord + PartialOrd + Storable + BoundedStorable> TaskTimerPartition<T> {
    pub fn init(partition_manager: &mut StableMemory, id: u8) -> Self {
        let timer = partition_manager.init_min_heap("__timer", id).unwrap();

        Self(timer)
    }

    pub fn details(&self) -> PartitionDetail {
        PartitionDetail {
            name: "__timer".to_string(),
            len: self.0.len(),
        }
    }

    pub fn timers(&self) -> &TaskTimerPartition<T> {
        &self
    }

    pub fn timers_mut(&mut self) -> &mut TaskTimerPartition<T> {
        self
    }

    pub fn get_timers(&self) -> Vec<TaskTimerEntry<T>> {
        self.0.iter().collect()
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

    pub fn peek_timer(&self) -> Option<TaskTimerEntry<T>> {
        self.0.peek()
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

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn test_timer_entry_to_and_from_bytes() {
        #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
        enum TestTask {
            A,
            B,
            C(String),
        }

        impl Storable for TestTask {
            fn to_bytes(&self) -> Cow<[u8]> {
                match self {
                    TestTask::A => 9876543210u64.to_bytes(),
                    TestTask::B => 1234567890u64.to_bytes(),
                    TestTask::C(s) => s.as_bytes().to_vec().into(),
                }
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                match bytes.len() {
                    8 => {
                        let value = u64::from_bytes(bytes);
                        if value == 9876543210 {
                            TestTask::A
                        } else {
                            TestTask::B
                        }
                    }
                    _ => TestTask::C(String::from_utf8(bytes.to_vec()).unwrap()),
                }
            }
        }

        impl BoundedStorable for TestTask {
            const IS_FIXED_SIZE: bool = true;
            const MAX_SIZE: u32 = 24;
        }

        let entry = TaskTimerEntry {
            time: 1234567890.into(),
            task: TestTask::A,
        };

        let bytes = entry.to_bytes();
        assert_eq!(bytes.len(), 16);

        let entry_from_bytes = TaskTimerEntry::from_bytes(bytes);

        assert_eq!(entry, entry_from_bytes);
        assert_eq!(entry_from_bytes.time, 1234567890.into());
        assert_eq!(entry_from_bytes.task, TestTask::A);

        let entry = TaskTimerEntry {
            time: 1234567890.into(),
            task: TestTask::B,
        };

        let bytes = entry.to_bytes();
        assert_eq!(bytes.len(), 16);

        let entry_from_bytes = TaskTimerEntry::from_bytes(bytes);

        assert_eq!(entry, entry_from_bytes);
        assert_eq!(entry_from_bytes.time, 1234567890.into());

        let entry = TaskTimerEntry {
            time: 1234567890.into(),
            task: TestTask::C("Hello World!".to_string()),
        };

        let bytes = entry.to_bytes();
        assert!(bytes.len() < size_of::<TaskTimerEntry<TestTask>>());

        let entry_from_bytes = TaskTimerEntry::from_bytes(bytes);

        assert_eq!(entry, entry_from_bytes);
        assert_eq!(entry_from_bytes.time, 1234567890.into());
        assert_eq!(
            entry_from_bytes.task,
            TestTask::C("Hello World!".to_string())
        );
    }
}
