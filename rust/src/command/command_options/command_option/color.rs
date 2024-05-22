use crate::{
    CommandOptionArg,
    CommandOptionArgInternal,
    Color,
};
use std::fmt::Display;


pub const NAME: &str = "color";


#[derive(Clone)]
pub struct Arg(ValueType);
pub(crate) type ValueType = Color;

impl CommandOptionArg for Arg {
    type ValueType = ValueType;

    fn value(&self) -> Self::ValueType {
        self.0.clone()
    }
}

impl CommandOptionArgInternal for Arg {
    fn new_infallible(value: Self::ValueType) -> Self {
        Self(value)
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
