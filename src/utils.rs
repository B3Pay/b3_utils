mod hasher;
mod ic;

pub use hasher::*;
pub use ic::*;

#[macro_export]
macro_rules! require {
    ($condition:expr, $($msg:tt)*) => {
        if !$condition {
            return Err(format!($($msg)*));
        }
    };
}
