use crate::{
    ErrorLevel,
    Validate,
};
use std::fmt::Display;


pub const NAME: &str = "errorlevel";


#[derive(Clone)]
pub struct Arg(ArgType);
pub(crate) type ArgType = ErrorLevel;

impl Validate for Arg {
    type ValueType = ArgType;

    fn new_infallible(value: Self::ValueType) -> Self {
        Self(value)
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
