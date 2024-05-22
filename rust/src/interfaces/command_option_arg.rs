use crate::Error;


pub trait CommandOptionArg {
    type ValueType;

    fn value(&self) -> Self::ValueType;
}


pub(crate) trait CommandOptionArgInternal: CommandOptionArg + Sized {
    fn new(value: Self::ValueType) -> Result<Self, Error> {
        Self::validate(&value)?;
        Ok(Self::new_infallible(value))
    }

    fn new_infallible(value: Self::ValueType) -> Self;

    fn validate(_value: &Self::ValueType) -> Result<(), Error> {
        Ok(())
    }
}
