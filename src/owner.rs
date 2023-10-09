use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::cell::ValueError;

use crate::{
    memory::{error::StableMemoryError, types::DefaultVMCell, with_stable_mem_mut},
    Subaccount,
};

fn init_owner(name: &str, id: u8) -> Result<RefCell<DefaultVMCell<Subaccount>>, StableMemoryError> {
    let memory = with_stable_mem_mut(|pm| pm.create(name, id))?;

    let owner_id = Subaccount::from(ic_cdk::caller());

    let cell = DefaultVMCell::init(memory, owner_id)
        .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))?;

    Ok(RefCell::new(cell))
}

thread_local! {
    static OWNER: RefCell<DefaultVMCell<Subaccount>> = init_owner("owner", 253).unwrap();
}

pub fn get_owner() -> Principal {
    OWNER.with(|states| {
        let state = states.borrow();

        state.get().to_principal().unwrap()
    })
}

pub fn set_owner(new_owner: Principal) -> Result<Principal, ValueError> {
    OWNER.with(|states| {
        let mut state = states.borrow_mut();
        let old_owner = state.set(new_owner.into())?;

        Ok(old_owner.to_principal().unwrap())
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
