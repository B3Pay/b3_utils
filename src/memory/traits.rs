use super::{
    error::StableMemoryError,
    timer::DefaultTaskTimer,
    types::{
        DefaultStableBTreeMap, DefaultStableCell, DefaultStableLog, DefaultStableMinHeap,
        DefaultStableVec, DefaultVM,
    },
};

pub use ic_stable_structures::{
    cell::InitError as ExternalCellInitError, log::InitError as ExternalLogInitError,
    memory_manager::VirtualMemory, DefaultMemoryImpl, FileMemory, Memory, RestrictedMemory,
    StableBTreeMap, StableCell, StableLog, StableMinHeap, StableVec, Storable, VectorMemory,
};

#[rustfmt::skip]
pub enum InitMemoryArg {
    Single(DefaultVM),
    Double(DefaultVM, DefaultVM),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryType {
    Map,
    Vec,
    Log,
    Cell,
    Heap,
    Timer,
}

pub trait InitMemory<T>: Sized {
    fn memory_type() -> MemoryType;
    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError>;
}

impl<T: Storable> InitMemory<DefaultStableVec<T>> for DefaultStableVec<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Vec
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            StableVec::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<K: Ord + Storable + Clone, V: Storable> InitMemory<DefaultStableBTreeMap<K, V>>
    for DefaultStableBTreeMap<K, V>
{
    fn memory_type() -> MemoryType {
        MemoryType::Map
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            Ok(StableBTreeMap::init(memory))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Storable> InitMemory<DefaultStableLog<T>> for DefaultStableLog<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Log
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Double(index_memory, data_memory) = arg {
            StableLog::init(index_memory, data_memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Storable + Default> InitMemory<DefaultStableCell<T>> for DefaultStableCell<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Cell
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            StableCell::init(memory, T::default())
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Ord + Storable> InitMemory<DefaultStableMinHeap<T>> for DefaultStableMinHeap<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Heap
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            DefaultStableMinHeap::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Storable + Clone> InitMemory<DefaultTaskTimer<T>> for DefaultTaskTimer<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Timer
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            DefaultTaskTimer::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

pub trait StateManagement {
    type Item;
    type View;
    type Error;
    type AddArgs;
    type WriteState;
    type ReadState;
    type Id;

    /// Add a new item in the state
    fn add(args: Self::AddArgs) -> Result<Self::Id, Self::Error>;

    /// Get a writable state for an item
    fn write(id: Self::Id) -> Self::WriteState;

    /// Get a readable state for an item
    fn read(id: Self::Id) -> Self::ReadState;

    /// Iterate over all items in the state
    fn iter<F, R>(f: F) -> Vec<R>
    where
        F: FnMut(&Self::Id, &Self::Item) -> R;

    /// Get views of all items in the state
    fn views() -> Vec<Self::View>;

    /// Get the number of items in the state
    fn len() -> u64;

    /// Reset the entire state
    fn reset();
}
