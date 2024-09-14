use std::cell::RefCell;

use candid::Principal;

use crate::{
    memory::{error::StableMemoryError, types::DefaultStableCell, with_stable_mem_mut},
    principal::StoredPrincipal,
};

fn init_owner(
    name: &str,
    id: u8,
) -> Result<RefCell<DefaultStableCell<StoredPrincipal>>, StableMemoryError> {
    let memory = with_stable_mem_mut(|pm| pm.create(name, id))?;

    let owner_id = StoredPrincipal::from(ic_cdk::caller());

    let cell = DefaultStableCell::init(memory, owner_id)
        .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))?;

    Ok(RefCell::new(cell))
}

thread_local! {
    static OWNER: RefCell<DefaultStableCell<StoredPrincipal>> = init_owner("owner", 251).unwrap();
}

pub fn get_owner() -> Principal {
    OWNER.with(|states| {
        let state = states.borrow();

        state.get().into()
    })
}

pub fn set_owner(new_owner: Principal) -> Result<Principal, String> {
    OWNER.with(|states| {
        let mut state = states.borrow_mut();
        let old_owner = state.set(new_owner.into()).expect("Unable to set owner");

        Ok(old_owner.into())
    })
}

pub fn caller_is_owner() -> Result<(), String> {
    let caller_id = ic_cdk::caller();
    let owner_id = get_owner();

    if caller_id == owner_id {
        Ok(())
    } else {
        Err(format!("Caller is not the owner. Caller: {}", caller_id))
    }
}
