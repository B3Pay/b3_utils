use std::cell::RefCell;

use b3_utils::{
    memory::{
        init_stable_mem_refcell,
        timer::DefaultTaskTimer,
        types::{Bound, DefaultVMMap, Storable},
    },
    Subaccount,
};
use candid::CandidType;
use candid::{decode_one, encode_one};
use serde::{Deserialize, Serialize};

thread_local! {
    static TASK_TIMER: RefCell<DefaultTaskTimer<Task>> = init_stable_mem_refcell("timer", 1).unwrap();

    static USERS: RefCell<DefaultVMMap<u64, Subaccount>> = init_stable_mem_refcell("users", 2).unwrap();
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Task {
    AddUser(Subaccount),
    RemoveUser(u64),
}

impl Storable for Task {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        decode_one(&bytes).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        encode_one(self).unwrap().into()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[ic_cdk::update]
fn add_user() {
    let principal = ic_cdk::caller();

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        let user_id = users.len() as u64;

        users.insert(user_id, principal.into());
    });
}
