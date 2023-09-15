use candid::CandidType;

pub use b3_stable_structures::{
    cell::InitError as ExternalCellInitError, log::InitError as ExternalLogInitError,
    memory_manager::VirtualMemory, BoundedStorable, DefaultMemoryImpl, FileMemory, Memory,
    RestrictedMemory, StableBTreeMap, StableCell, StableLog, StableMinHeap, StableVec, Storable,
    VectorMemory,
};

use super::error::StableMemoryError;

#[derive(CandidType, Clone, Debug)]
pub struct PartitionDetail {
    pub name: String,
    pub len: u64,
}

pub type DefaultVM = VirtualMemory<DefaultMemoryImpl>;

pub type DefaultVMMap<K, V> = StableBTreeMap<K, V, DefaultVM>;
pub type DefaultVMVec<T> = StableVec<T, DefaultVM>;
pub type DefaultVMLog<T> = StableLog<T, DefaultVM, DefaultVM>;
pub type DefaultVMCell<T> = StableCell<T, DefaultVM>;
pub type DefaultVMHeap<T> = StableMinHeap<T, DefaultVM>;

#[rustfmt::skip]
pub enum InitArg {
    Map(DefaultVM),
    Vec(DefaultVM),
    Log(DefaultVM, DefaultVM),
    Cell(DefaultVM),
    Heap(DefaultVM),
}

pub trait InitTrait<T>: Sized {
    fn init(arg: InitArg) -> Result<Self, StableMemoryError>;
}

impl<T: Storable + BoundedStorable> InitTrait<DefaultVMVec<T>> for DefaultVMVec<T> {
    fn init(arg: InitArg) -> Result<Self, StableMemoryError> {
        if let InitArg::Vec(memory) = arg {
            StableVec::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<K: Ord + Storable + BoundedStorable + Clone, V: Storable + BoundedStorable>
    InitTrait<DefaultVMMap<K, V>> for DefaultVMMap<K, V>
{
    fn init(arg: InitArg) -> Result<Self, StableMemoryError> {
        if let InitArg::Map(memory) = arg {
            Ok(StableBTreeMap::init(memory))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Storable> InitTrait<DefaultVMLog<T>> for DefaultVMLog<T> {
    fn init(arg: InitArg) -> Result<Self, StableMemoryError> {
        if let InitArg::Log(index_memory, data_memory) = arg {
            StableLog::init(index_memory, data_memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Storable + Default> InitTrait<DefaultVMCell<T>> for DefaultVMCell<T> {
    fn init(arg: InitArg) -> Result<Self, StableMemoryError> {
        if let InitArg::Cell(memory) = arg {
            StableCell::init(memory, T::default())
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Ord + Storable + BoundedStorable> InitTrait<DefaultVMHeap<T>> for DefaultVMHeap<T> {
    fn init(arg: InitArg) -> Result<Self, StableMemoryError> {
        if let InitArg::Heap(memory) = arg {
            DefaultVMHeap::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}
