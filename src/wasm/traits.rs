use super::Wasm;
use serde_bytes::ByteBuf;

use crate::memory::types::{Bound, Storable};

impl Storable for Wasm {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(ByteBuf::from(bytes))
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_vec().into()
    }
}
