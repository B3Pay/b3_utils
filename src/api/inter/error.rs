#[derive(Debug)]
pub enum InterCallError {
    SerializationError(String, String),
    CallError(String, String),
}

#[rustfmt::skip]
impl std::fmt::Display for InterCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterCallError::SerializationError(method, msg) => 
                write!(f, "Error serializing arguments for method {}: {}", method, msg),
            InterCallError::CallError(method, msg) => 
                write!(f, "Error calling method {} : {}", method, msg),
        }
    }
}
