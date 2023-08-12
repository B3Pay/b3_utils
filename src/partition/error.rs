#[derive(Debug)]
pub enum PartitionError {
    PartitionExists,
    IdAlreadyUsed(String),
    UnableToCreateMemory(String),
}

#[rustfmt::skip]
impl std::fmt::Display for PartitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PartitionError::PartitionExists => write!(f, "Partition already exists"),
            PartitionError::IdAlreadyUsed(name) => write!(f, "ID already used for partition {}", name),
            PartitionError::UnableToCreateMemory(err) => write!(f, "Unable to create memory: {:?}", err.to_string())
        }
    }
}
