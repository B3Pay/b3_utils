use std::cell::RefCell;

use super::Wasm;

thread_local! {
    static WASM_CACHE: RefCell<Wasm> = RefCell::default();
}

/// Get wasm.
pub fn with_wasm_cache<T, F>(callback: F) -> T
where
    F: FnOnce(&Wasm) -> T,
{
    WASM_CACHE.with(|wasm| {
        let wasm = wasm.borrow();

        callback(&wasm)
    })
}

/// Get wasm mutably.
pub fn with_wasm_mut_cache<T, F>(callback: F) -> T
where
    F: FnOnce(&mut Wasm) -> T,
{
    WASM_CACHE.with(|wasm| {
        let mut wasm = wasm.borrow_mut();

        callback(&mut wasm)
    })
}
