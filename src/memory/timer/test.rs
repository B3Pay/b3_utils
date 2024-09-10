#[cfg(test)]
mod tests {
    use std::{borrow::Cow, mem::size_of};

    use ic_stable_structures::{storable::Bound, Storable};

    use crate::memory::timer::TaskTimerEntry;

    #[test]
    fn test_timer_entry_to_and_from_bytes() {
        #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
        enum TestTask {
            A,
            B,
            C(String),
        }

        impl Storable for TestTask {
            const BOUND: Bound = Bound::Bounded {
                is_fixed_size: false,
                max_size: 8,
            };

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

        let entry = TaskTimerEntry {
            time: 1234567890.into(),
            task: TestTask::A,
            interval: None,
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
            interval: None,
        };

        let bytes = entry.to_bytes();
        assert_eq!(bytes.len(), 16);

        let entry_from_bytes = TaskTimerEntry::from_bytes(bytes);

        assert_eq!(entry, entry_from_bytes);
        assert_eq!(entry_from_bytes.time, 1234567890.into());

        let entry = TaskTimerEntry {
            time: 1234567890.into(),
            task: TestTask::C("Hello World!".to_string()),
            interval: None,
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
