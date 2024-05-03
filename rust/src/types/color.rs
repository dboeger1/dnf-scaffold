use std::fmt::Display;


pub const STR_AUTO: &str = "auto";
pub const STR_ALWAYS: &str = "always";
pub const STR_NEVER: &str = "never";


#[derive(Clone)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl Default for Color {
    fn default() -> Self {
        Self::Auto
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Auto => STR_AUTO,
                Self::Always => STR_ALWAYS,
                Self::Never => STR_NEVER,
            },
        )
    }
}
