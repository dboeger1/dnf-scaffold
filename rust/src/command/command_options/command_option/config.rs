use crate::{
    Error,
    Validate,
};
use std::{
    fmt::Display,
    path::PathBuf,
};


pub const NAME: &str = "config";


#[derive(Clone)]
pub struct Arg(ArgType);
pub(crate) type ArgType = PathBuf;

impl Validate for Arg {
    type ValueType = ArgType;

    fn new_infallible(value: Self::ValueType) -> Self {
        Self(value)
    }

    fn validate(_value: &Self::ValueType) -> Result<(), Error> {
        if _value.to_string_lossy().is_empty() {
            return Err(Error {
                message: format!("--{NAME} requires non-empty argument."),
                source: None,
            });
        }

        Ok(())
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}
