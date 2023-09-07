use core::fmt;

use candid::CandidType;

pub use ic_stable_structures::{
    cell::InitError as ExternalCellInitError, log::InitError as ExternalLogInitError,
    memory_manager::VirtualMemory, BoundedStorable, DefaultMemoryImpl, FileMemory, Memory,
    RestrictedMemory, StableBTreeMap, StableCell, StableLog, StableMinHeap, StableVec, Storable,
    VectorMemory,
};

use crate::Subaccount;

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

impl Storable for Subaccount {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Subaccount::from_slice(&bytes).unwrap()
    }
}

impl BoundedStorable for Subaccount {
    const IS_FIXED_SIZE: bool = true;
    const MAX_SIZE: u32 = 32;
}

pub struct LogInitError(pub ExternalLogInitError);

impl fmt::Display for LogInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ExternalLogInitError::IncompatibleDataVersion {
                last_supported_version,
                decoded_version,
            } => write!(
                f,
                "Incompatible data version: last supported version is {}, but decoded version is {}",
                last_supported_version, decoded_version
            ),
            ExternalLogInitError::IncompatibleIndexVersion {
                last_supported_version,
                decoded_version,
            } => write!(
                f,
                "Incompatible index version: last supported version is {}, but decoded version is {}",
                last_supported_version, decoded_version
            ),
            ExternalLogInitError::InvalidIndex => write!(f, "Invalid index"),
        }
    }
}

pub struct CellInitError(pub ExternalCellInitError);

impl fmt::Display for CellInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ExternalCellInitError::IncompatibleVersion {
                last_supported_version,
                decoded_version,
            } => write!(
                f,
                "Incompatible version: last supported version is {}, but the memory contains version {}",
                last_supported_version, decoded_version
            ),
            ExternalCellInitError::ValueTooLarge { value_size } => write!(
                f,
                "The initial value is too large to fit into the memory: {} bytes",
                value_size
            ),
        }
    }
}
