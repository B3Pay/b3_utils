use candid::CandidType;
use serde::{Deserialize, Serialize};

pub use ic_stable_structures::{
    cell::InitError as ExternalCellInitError, log::InitError as ExternalLogInitError,
    memory_manager::VirtualMemory, storable::Bound, DefaultMemoryImpl, FileMemory, Memory,
    RestrictedMemory, StableBTreeMap, StableCell, StableLog, StableMinHeap, StableVec, Storable,
    VectorMemory,
};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct PartitionDetail {
    pub name: String,
    pub size: u64,
    pub id: u8,
}

pub type DefaultVM = VirtualMemory<DefaultMemoryImpl>;

pub type DefaultStableBTreeMap<K, V> = StableBTreeMap<K, V, DefaultVM>;
pub type DefaultStableVec<T> = StableVec<T, DefaultVM>;
pub type DefaultStableLog<T> = StableLog<T, DefaultVM, DefaultVM>;
pub type DefaultStableCell<T> = StableCell<T, DefaultVM>;
pub type DefaultStableMinHeap<T> = StableMinHeap<T, DefaultVM>;
