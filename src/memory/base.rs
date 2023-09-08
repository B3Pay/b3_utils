use b3_stable_structures::Memory;

use super::types::PartitionDetail;
use super::StableMemory;

mod test;

pub mod backup;

mod store;
pub use store::*;

use backup::MainBackupType;

pub struct BasePartition {
    backup: MainBackupType,
}

impl BasePartition {
    /// Initializes the core partition.
    /// The core partition is composed of 3 sub-partitions:
    /// - __backup 0
    /// - __timer 1
    ///
    /// The backup partition is used to store the backup state of the canister.
    /// The events_data and events_index partitions are used to store the events of the canister.
    pub fn init(partition_manager: &mut StableMemory) -> Self {
        let backup = partition_manager.create("__backup", 0).unwrap();

        Self { backup }
    }

    pub fn backup_details(&self) -> PartitionDetail {
        PartitionDetail {
            name: "__backup".to_string(),
            len: self.backup.size(),
        }
    }
}
