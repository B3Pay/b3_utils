use super::types::DefaultVM;
use crate::{memory::DefaultStableMinHeap, NanoTimeStamp};
use candid::CandidType;
use ic_stable_structures::{storable::Bound, vec::InitError, GrowFailed, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cmp::Ordering};

mod test;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct TaskTimerEntry<T> {
    pub time: NanoTimeStamp,
    pub task: T,
    pub interval: Option<NanoTimeStamp>,
}

pub struct DefaultTaskTimer<T: Storable>(DefaultStableMinHeap<TaskTimerEntry<T>>);

impl<T: Storable + Clone> DefaultTaskTimer<T> {
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
        let timer = self.0.pop();
        if let Some(timer) = timer.clone() {
            if let Some(interval) = timer.interval {
                // If it's an interval timer, reschedule it
                let new_time = timer.time + interval.clone();
                let new_timer = TaskTimerEntry {
                    time: new_time,
                    task: timer.task.clone(),
                    interval: Some(interval),
                };
                let _ = self.push_timer(&new_timer);
            }
        }

        timer
    }

    pub fn clear_timer(&mut self) {
        while self.0.pop().is_some() {}
    }

    // New method to set an interval timer
    pub fn set_timer_interval(
        &mut self,
        interval: NanoTimeStamp,
        task: T,
    ) -> Result<(), GrowFailed> {
        let now = ic_cdk::api::time();
        let timer = TaskTimerEntry {
            time: NanoTimeStamp::from(interval.0 + now),
            task,
            interval: Some(interval),
        };
        self.push_timer(&timer)
    }
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
        let interval_bytes = match &self.interval {
            Some(interval) => interval.to_le_bytes().to_vec(),
            None => vec![0; 16], // Use 16 bytes for consistency
        };
        let total_size = 8 + task_bytes.len() + 16;
        let mut bytes = vec![0; total_size];
        bytes[0..8].copy_from_slice(&time_bytes);
        bytes[8..8 + task_bytes.len()].copy_from_slice(&task_bytes);
        bytes[8 + task_bytes.len()..].copy_from_slice(&interval_bytes);
        bytes.into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let time = NanoTimeStamp::from_le_bytes(bytes[0..8].try_into().unwrap());
        let task_end = bytes.len() - 16;
        let task = T::from_bytes(bytes[8..task_end].into());
        let interval = if bytes[task_end..] == [0; 16] {
            None
        } else {
            let nanos = u128::from_le_bytes(bytes[task_end..].try_into().unwrap());
            Some(NanoTimeStamp(nanos as u64))
        };
        Self {
            time,
            task,
            interval,
        }
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: false,
        max_size: 24 + T::BOUND.max_size(), // 8 for time, 16 for interval, plus task size
    };
}
