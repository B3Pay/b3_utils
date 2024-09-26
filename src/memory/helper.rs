/// Trait for state-reading operations
pub trait StateAccess {
    type Item;
    type View;
    type ReadState;
    type Id;

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
}

/// Trait for state-modifying operations
pub trait StateMutations: StateAccess {
    type Error;
    type AddArgs;
    type WriteState;

    /// Add a new item to the state
    fn add(args: Self::AddArgs) -> Result<Self::Id, Self::Error>;

    /// Get a writable state for an item
    fn write(id: Self::Id) -> Self::WriteState;

    /// Reset the entire state
    fn reset();
}
