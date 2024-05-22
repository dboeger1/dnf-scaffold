use crate::{
    CommandOptionArg,
    CommandOptionArgInternal,
    Error,
};
use std::fmt::Display;


pub const NAME: &str = "enableplugin";


#[derive(Clone)]
pub struct Arg(ValueType);
pub(crate) type ValueType = String;

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
