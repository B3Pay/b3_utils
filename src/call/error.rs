#[derive(Debug)]
pub enum InterCallError {
    CallError(String, String),
}

#[rustfmt::skip]
impl std::fmt::Display for InterCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterCallError::CallError(method, msg) => write!(f, "Error calling method {}: {}", method, msg),
        }
    }
}
