mod encoder;
mod ic;

use std::fmt;

pub use encoder::*;
pub use ic::*;

/// Macro to enforce a condition and return an error if it fails.
///
/// # Example
/// ```rust
/// use b3_utils::require;
///
/// fn example_function(x: u32, y: u32) -> Result<(), String> {
///     require!(x < y, "x must be less than y");
///     Ok(())
/// }
///
/// assert_eq!(example_function(1, 2), Ok(()));
/// assert_eq!(example_function(2, 1), Err("x must be less than y".to_string()));
/// ```
#[macro_export]
macro_rules! require {
    ($condition:expr, $($msg:tt)*) => {
        if !$condition {
            return Err(format!($($msg)*));
        }
    };
}

/// Reports an error by converting it to a string.
///
/// # Example
/// ```
/// use b3_utils::report;
///
/// fn example_function() -> Result<(), String> {
///    if(true) {
///       return report("An error occurred");
///    }
///
///    Ok(())
/// }
///
/// assert_eq!(example_function(), Err("An error occurred".to_string()));
/// ```
pub fn report<T, E: fmt::Display>(err: E) -> Result<T, String> {
    Err(format!("{}", err))
}

/// Reports and logs an error (only when the "logging" feature is enabled).
///
/// # Example
/// ```
/// use b3_utils::{report_log, logs::export_log};
///
/// fn example_function() -> Result<(), String> {
///   if(true) {
///      return report_log("An error occurred");
///   }
///
///   Ok(())
/// }
///
/// assert_eq!(example_function(), Err("An error occurred".to_string()));
/// assert_eq!(export_log()[0].message, "An error occurred");
/// ```
#[cfg(feature = "logging")]
pub fn report_log<T, E: fmt::Display>(err: E) -> Result<T, String> {
    crate::log!("{}", err);

    Err(format!("{}", err))
}

/// Logs and panics with the given error message (only when the "logging" feature is enabled).
///
/// # Example
/// ```rust
/// use b3_utils::{panic_log, logs::export_log};
///
/// fn example_function() -> Result<(), String> {
///    if(true) {
///      return panic_log("An error occurred");
///    }
///
///    Ok(())
/// }
///
/// let result = std::panic::catch_unwind(|| example_function());
///
/// assert!(result.is_err());
///
/// assert_eq!(export_log()[0].message, "An error occurred");
/// ```
#[cfg(feature = "logging")]
pub fn panic_log<T, E: fmt::Display>(err: E) -> T {
    crate::log!("{}", err);

    panic!("{}", err);
}
