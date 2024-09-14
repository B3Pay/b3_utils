use std::fmt;

pub fn caller_is_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        Ok(())
    } else {
        Err("Caller is not controller".to_string())
    }
}

pub fn revert<T, E: fmt::Display>(err: E) -> T {
    ic_cdk::trap(&format!("{}", err));
}
