use candid::CandidType;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

mod test;

mod store;
pub use store::*;

mod utils;
pub use utils::*;

mod types;
pub use types::*;

pub mod traits;

/// Represents a WebAssembly (Wasm) binary.
#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Wasm(pub ByteBuf);

impl Default for Wasm {
    fn default() -> Self {
        Self(ByteBuf::new())
    }
}

impl Wasm {
    /// Loads a Wasm binary blob into the current Wasm instance.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    /// use b3_utils::wasm::Blob;
    ///
    /// let mut wasm = Wasm::default();
    /// let blob: Blob = vec![0x00, 0x61, 0x73, 0x6D];
    /// wasm.load(&blob);
    /// ```
    pub fn load(&mut self, blob: &Blob) -> WasmSize {
        self.0.extend(blob);

        self.0.len()
    }

    /// Unloads the current Wasm binary.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// wasm.unload();
    /// ```
    pub fn unload(&mut self) -> WasmSize {
        self.0.clear();

        self.0.len()
    }

    /// Returns the size of the current Wasm binary.
    /// The size of the current Wasm binary is equal to the number of bytes in the current Wasm binary.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert_eq!(wasm.len(), 0);
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// assert_eq!(wasm.len(), 4);
    /// ```
    pub fn len(&self) -> WasmSize {
        self.0.len()
    }

    /// Returns the current Wasm binary.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    /// use b3_utils::wasm::Blob;
    ///
    /// let mut wasm = Wasm::default();
    /// assert_eq!(wasm.bytes(), Blob::default());
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// assert_eq!(wasm.bytes(), Blob::from(vec![0x00, 0x61, 0x73, 0x6D]));
    /// ```
    pub fn bytes(&self) -> Blob {
        self.0.to_vec()
    }
    /// Returns true if the current Wasm binary is empty.
    /// The current Wasm binary is empty if its size is equal to zero.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert!(wasm.is_empty());
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// assert!(!wasm.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Returns true if the current Wasm binary is loading.
    /// The current Wasm binary is loading if it is empty or if its size is less than the given size.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert!(wasm.is_loading(0));
    /// assert!(wasm.is_loading(1));
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// assert!(wasm.is_loading(5));
    /// ```
    pub fn is_loading(&self, size: usize) -> bool {
        self.0.is_empty() || self.0.len() < size
    }

    /// Returns true if the current Wasm binary is loaded.
    /// The current Wasm binary is loaded if its size is equal to the given size.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert!(!wasm.is_loaded(1));
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// assert!(wasm.is_loaded(4));
    /// ```
    pub fn is_loaded(&self, size: usize) -> bool {
        self.0.len() == size
    }

    /// Returns true if the current Wasm binary is unloaded.
    /// The current Wasm binary is unloaded if its size is equal to zero.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert!(wasm.is_unloaded());
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// assert!(!wasm.is_empty());
    /// wasm.unload();
    /// assert!(wasm.is_unloaded());
    /// ```
    pub fn is_unloaded(&self) -> bool {
        self.0.is_empty()
    }

    /// Generates a hash of the current Wasm binary.
    /// Returns a default hash if the current Wasm binary is empty.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert_eq!(wasm.hash().to_vec(), vec![0x00; 32]);
    ///
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// let hash = wasm.hash();
    /// assert_eq!(hash.to_vec(), vec![205, 93, 73, 53, 164, 140, 6, 114, 203, 6, 64, 123, 180, 67, 188, 0, 135, 175, 249, 71, 198, 184, 100, 186, 200, 134, 152, 44, 115, 179, 2, 127]);
    /// ```
    pub fn hash(&self) -> WasmHash {
        if self.0.is_empty() {
            return WasmHash::default();
        }

        sha256_wasm_hash(&self.0)
    }

    /// Generates a hash string of the current Wasm binary.
    /// Returns a default hash string if the current Wasm binary is empty.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert_eq!(wasm.hash_string(), String::default());
    ///
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// let hash_string = wasm.hash_string();
    /// assert_eq!(hash_string, "cd5d4935a48c0672cb06407bb443bc0087aff947c6b864bac886982c73b3027f");
    /// ```
    pub fn hash_string(&self) -> String {
        if self.0.is_empty() {
            return String::default();
        }

        sha256_wasm_hash_string(&self.0)
    }

    /// Verifies the given hash against the current Wasm binary.
    /// Returns true if the given hash matches the current Wasm binary.
    ///
    /// # Example
    /// ```
    /// use b3_utils::wasm::Wasm;
    ///
    /// let mut wasm = Wasm::default();
    /// assert!(wasm.verify_hash(&[0x00; 32]));
    ///
    /// wasm.load(&vec![0x00, 0x61, 0x73, 0x6D]);
    /// assert!(wasm.verify_hash(&[205, 93, 73, 53, 164, 140, 6, 114, 203, 6, 64, 123, 180, 67, 188, 0, 135, 175, 249, 71, 198, 184, 100, 186, 200, 134, 152, 44, 115, 179, 2, 127]));
    /// ```
    pub fn verify_hash(&self, hash: &WasmHash) -> bool {
        self.hash() == *hash
    }
}
