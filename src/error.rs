use std::fmt;

struct StarkBankError {
    message: String,
}
struct InternalServerError {
    message: String,
}

#[derive(Debug)]
struct Error {
    code: String,
    message: String,
}

struct InputErrors {
    content: Vec<Error>,
}

struct UnknownError {
    message: String,
}

struct InvalidSignatureError {
    message: String,
}

impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown exception encountered: {}", self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl fmt::Display for InputErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.content)
    }
}

macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.message)
            }
        })*
    }
}

impl_T!(for StarkBankError, InternalServerError, InvalidSignatureError);
