#[derive(Debug)]
pub enum StableMemoryError {
    PartitionExists,
    InvalidMemoryType,
    UnknownType,
    WrongInitializationArgument,
    IdAlreadyUsed(String),
    IdOutOfRange(u8),
    UnableToCreateMemory(String),
}

#[rustfmt::skip]
impl std::fmt::Display for StableMemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StableMemoryError::PartitionExists => write!(f, "Partition already exists"),
            StableMemoryError::InvalidMemoryType => write!(f, "Invalid memory type"),
            StableMemoryError::UnknownType => write!(f, "Unknown type"),
            StableMemoryError::WrongInitializationArgument => write!(f, "Wrong initialization argument"),
            StableMemoryError::IdOutOfRange(id) => write!(f, "Wrong ID {} - must be between 1 and 250", id),
            StableMemoryError::IdAlreadyUsed(name) => write!(f, "ID already used for partition {}", name),
            StableMemoryError::UnableToCreateMemory(err) => write!(f, "Unable to create memory: {:?}", err.to_string())
        }
    }
}
