use candid::CandidType;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::thread::LocalKey;

pub mod counter;

mod store;
pub use store::*;

mod test;

/// An entry in the canister log.
#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogEntry {
    pub timestamp: u64,
    // The index of this entry starting from the last canister upgrade.
    pub counter: u64,
    pub message: String,
    pub file: &'static str,
    pub line: u32,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}:{} {}",
            self.timestamp, self.file, self.line, self.message
        )
    }
}

/// A circular buffer for log messages.
pub struct LogBuffer {
    max_capacity: usize,
    entries: VecDeque<LogEntry>,
}

impl LogBuffer {
    /// Creates a new buffer of the specified max capacity.
    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            entries: VecDeque::with_capacity(max_capacity),
        }
    }

    /// Adds a new entry to the buffer, potentially evicting older entries.
    pub fn append(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    /// Returns an iterator over entries in the order of their insertion.
    pub fn iter(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }

    /// Returns the first iterator for which p returns false (or past the end
    /// iterator if they all return false).
    ///
    /// Warning: Entries MUST be partitioned by p. That is, p returns true for
    /// all elements in the "front" of the entries list, and false for all
    /// elements in the "back". Otherwise, behavior is not defined. This is
    /// because binary search is used.
    ///
    /// For example,
    ///
    ///   log_buffer.skip_old_entries(|log_entry| log_entry.timestamp <= T)
    ///
    /// In practice, p only uses the timestamp field, because you can partition
    /// on that (since entries are in chronological order, assuming the clock is
    /// monotonic, and the IC, it is).
    ///
    /// If you want an iterator to the first iterator that returns true, but p
    /// does not partition, do this instead:
    ///
    ///    log_buffer.iter().skip_while(opposite_of_p)
    pub fn entries_partition_point<P>(&self, p: P) -> impl Iterator<Item = &LogEntry>
    where
        P: Fn(&LogEntry) -> bool,
    {
        let head_len = self.entries.partition_point(p);
        self.iter().skip(head_len)
    }
}

pub type GlobalBuffer = LocalKey<RefCell<LogBuffer>>;

#[derive(Clone)]
pub struct DevNull;

pub trait Sink {
    fn append(&self, entry: LogEntry);
}

impl Sink for DevNull {
    fn append(&self, _: LogEntry) {}
}

impl Sink for &'static GlobalBuffer {
    fn append(&self, entry: LogEntry) {
        self.with(|cell| cell.borrow_mut().append(entry))
    }
}

mod private {
    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "ic0")]
    extern "C" {
        pub fn time() -> u64;
    }

    #[cfg(target_arch = "wasm32")]
    pub fn timestamp() -> u64 {
        unsafe { time() }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn timestamp() -> u64 {
        use std::time::SystemTime;

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_nanos() as u64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}

/// Returns the current time as a number of nanoseconds passed since the Unix
/// epoch.
#[doc(hidden)]
pub fn now() -> u64 {
    private::timestamp()
}
