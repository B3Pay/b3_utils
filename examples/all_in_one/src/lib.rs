use std::cell::RefCell;

use b3_utils::{
    memory::{
        init_stable_mem_refcell,
        timer::DefaultTaskTimer,
        types::{DefaultVMMap, Storable},
    },
    Subaccount,
};

thread_local! {
    static TASK_TIMER: RefCell<DefaultTaskTimer<Task>> = init_stable_mem_refcell("timer", 1).unwrap();

    static USERS: RefCell<DefaultVMMap<u64, Subaccount>> = init_stable_mem_refcell("users", 2).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Task {
    AddUser(Principal),
    RemoveUser(u64),
}

impl Storable for Task {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Task::AddUser(principal) => {
                let mut bytes = vec![0];
                bytes.extend(principal.as_slice());
                bytes
            }
            Task::RemoveUser(id) => {
                let mut bytes = vec![1];
                bytes.extend(id.to_be_bytes());
                bytes
            }
        }
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        match bytes[0] {
            0 => Task::AddUser(Principal::from_slice(&bytes[1..])),
            1 => Task::RemoveUser(u64::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]])),
            _ => panic!("Invalid task type"),
        }
    }
}

#[ic_cdk::update]
fn add_user() {
    let principal = ic_cdk::caller();

    TASK_TIMER.with(|task_timer| {
        task_timer.borrow_mut().add_task(Task::AddUser(principal));
    });
}
