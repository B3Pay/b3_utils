mod key;

use b3_stable_structures::btreemap::Iter;
pub use key::*;

use super::types::{DefaultVM, DefaultVMMap};

pub struct Partitions(DefaultVMMap<LimitedString, u8>);

impl Partitions {
    pub fn init(default_vm: DefaultVM) -> Self {
        Self(DefaultVMMap::init(default_vm))
    }

    pub fn get(&self, name: &LimitedString) -> Option<u8> {
        self.0.get(name)
    }

    pub fn insert(&mut self, name: LimitedString, id: u8) -> Option<u8> {
        self.0.insert(name, id)
    }

    pub fn iter(&self) -> Iter<LimitedString, u8, DefaultVM> {
        self.0.iter()
    }
}
