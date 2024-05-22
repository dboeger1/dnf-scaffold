use crate::{
    Exclude,
    Validate,
};
use std::fmt::Display;


pub const NAME: &str = "disableexcludes";


#[derive(Clone)]
pub struct Arg(ArgType);
pub(crate) type ArgType = Exclude;

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
