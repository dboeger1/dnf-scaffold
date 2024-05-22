use crate::{
    Error,
    Validate,
};
use std::fmt::Display;


pub const NAME: &str = "bz";


#[derive(Clone)]
pub struct Arg(ArgType);
pub(crate) type ArgType = String;

impl Validate for Arg {
    type ValueType = ArgType;

    fn new_infallible(value: Self::ValueType) -> Self {
        Self(value)
    }

    fn validate(_value: &Self::ValueType) -> Result<(), crate::Error> {
        if _value.is_empty() {
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
        self.0.fmt(f)
    }
}
