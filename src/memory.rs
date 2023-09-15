use b3_stable_structures::memory_manager::{MemoryId, MemoryManager};
use b3_stable_structures::{
    BoundedStorable, DefaultMemoryImpl, StableBTreeMap, StableCell, StableLog, StableVec, Storable,
};

mod test;

pub mod error;
use error::StableMemoryError;

pub mod backup;
pub mod timer;

mod store;
pub use store::*;

pub mod partitions;
pub mod types;

use types::{DefaultVM, DefaultVMCell, DefaultVMHeap, DefaultVMLog, DefaultVMMap, DefaultVMVec};

use self::backup::BackupPartition;
use self::partitions::{LimitedString, Partitions};
use self::types::{InitTrait, PartitionDetail};

pub struct StableMemory {
    memory_manager: MemoryManager<DefaultMemoryImpl>,
    backup: BackupPartition,
    partitions: Partitions,
}

impl StableMemory {
    pub fn init() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        let partitions_vm = memory_manager.get(MemoryId::new(254));
        let partitions = Partitions::init(partitions_vm);

        let backup_vm = memory_manager.get(MemoryId::new(0));
        let backup = BackupPartition::init(backup_vm);

        Self {
            memory_manager,
            partitions,
            backup,
        }
    }
    fn check_partition(&self, name: &LimitedString, id: u8) -> Result<(), StableMemoryError> {
        match self.partitions.get(&name) {
            Some(existing_id) if existing_id != id => {
                return Err(StableMemoryError::IdAlreadyUsed(name.to_string()));
            }
            Some(_) | None => {}
        }

        for (memory_name, memory_id) in self.partitions.iter() {
            if memory_id == id && memory_name != name.to_owned() {
                return Err(StableMemoryError::IdAlreadyUsed(memory_name.to_string()));
            }
        }

        Ok(())
    }

    pub fn create(&mut self, name: &str, id: u8) -> Result<DefaultVM, StableMemoryError> {
        let name = LimitedString::from(name);

        self.check_partition(&name, id)?;

        self.partitions.insert(name.clone(), id);

        let memory = self
            .memory(&name.to_string())
            .ok_or(StableMemoryError::UnableToCreateMemory(name.to_string()))?;

        Ok(memory)
    }

    pub fn backup(&self) -> &BackupPartition {
        &self.backup
    }

    pub fn backup_mut(&mut self) -> &mut BackupPartition {
        &mut self.backup
    }

    pub fn partition(&self, name: &str) -> Option<u8> {
        self.partitions.get(&name.into())
    }

    pub fn partitions(&self) -> &Partitions {
        &self.partitions
    }

    pub fn memory(&self, name: &str) -> Option<DefaultVM> {
        let memory_id = self.partitions.get(&name.into())?;

        let vm = self.memory_manager.get(MemoryId::new(memory_id));

        Some(vm)
    }

    pub fn memory_manager(&self) -> &MemoryManager<DefaultMemoryImpl> {
        &self.memory_manager
    }

    pub fn backup_details(&self) -> PartitionDetail {
        PartitionDetail {
            name: "__backup".to_string(),
            len: self.backup.size(),
        }
    }

    pub fn init_memory<T: InitTrait<T>>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<T, StableMemoryError> {
        // TODO! find a way to make this match from type that get from T
        // let arg = match T::default() {
        //     _: DefaultVMVec<_> => InitArg::Vec(self.create(name, id)?),
        //     _: DefaultVMMap<_, _> => InitArg::Map(self.create(name, id)?),
        //     _: DefaultVMLog<_> => InitArg::Log(self.create(&format!("{}_index", name), id)?, self.create(&format!("{}_data", name), id + 1)?),
        //     _: DefaultVMCell<_> => InitArg::Cell(self.create(name, id)?),
        //     _: DefaultVMHeap<_> => InitArg::Heap(self.create(name, id)?),
        //     _ => return Err(StableMemoryError::UnknownType),
        // };

        // T::init(arg)
    }

    pub fn init_vec<T: Storable + BoundedStorable>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<DefaultVMVec<T>, StableMemoryError> {
        let memory = self.create(name, id)?;

        StableVec::init(memory).map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_btree_map<
        K: Ord + Storable + BoundedStorable + Clone,
        V: Storable + BoundedStorable,
    >(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<DefaultVMMap<K, V>, StableMemoryError> {
        let memory = self.create(name, id)?;

        let map = StableBTreeMap::init(memory);

        Ok(map)
    }

    pub fn init_min_heap<T: Ord + Storable + BoundedStorable>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<DefaultVMHeap<T>, StableMemoryError> {
        let memory = self.create(name, id)?;

        DefaultVMHeap::init(memory)
            .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_log<T: Storable>(
        &mut self,
        name: &str,
        index_id: u8,
        data_id: u8,
    ) -> Result<DefaultVMLog<T>, StableMemoryError> {
        let index_memory = self.create(&format!("{}_index", name), index_id)?;
        let data_memory = self.create(&format!("{}_data", name), data_id)?;

        StableLog::init(index_memory, data_memory)
            .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_cell<T: Storable + Default>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<DefaultVMCell<T>, StableMemoryError> {
        let memory = self.create(name, id)?;

        StableCell::init(memory, T::default())
            .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
    }
}
