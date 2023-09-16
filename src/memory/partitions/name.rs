use std::{borrow::Cow, fmt};

use b3_stable_structures::{BoundedStorable, Storable};

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct PartitionName(String);

impl From<String> for PartitionName {
    fn from(value: String) -> Self {
        if value.len() > 20 {
            panic!("String too long!");
        } else {
            Self(value)
        }
    }
}

impl From<&str> for PartitionName {
    fn from(value: &str) -> Self {
        if value.len() > 20 {
            panic!("String too long!");
        } else {
            Self(value.to_string())
        }
    }
}

impl fmt::Display for PartitionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl Storable for PartitionName {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(self.0.as_bytes())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(String::from_utf8_lossy(&bytes).into_owned())
    }
}

impl BoundedStorable for PartitionName {
    const IS_FIXED_SIZE: bool = false;
    const MAX_SIZE: u32 = 20;
}
