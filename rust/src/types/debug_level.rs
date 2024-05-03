use std::fmt::Display;


pub const STR_00: &str = "0";
pub const STR_01: &str = "1";
pub const STR_02: &str = "2";
pub const STR_03: &str = "3";
pub const STR_04: &str = "4";
pub const STR_05: &str = "5";
pub const STR_06: &str = "6";
pub const STR_07: &str = "7";
pub const STR_08: &str = "8";
pub const STR_09: &str = "9";
pub const STR_10: &str = "10";


#[derive(Clone)]
pub enum DebugLevel {
    L00,
    L01,
    L02,
    L03,
    L04,
    L05,
    L06,
    L07,
    L08,
    L09,
    L10,
}

impl Default for DebugLevel {
    fn default() -> Self {
        Self::L02
    }
}

impl Display for DebugLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::L00 => STR_00,
                Self::L01 => STR_01,
                Self::L02 => STR_02,
                Self::L03 => STR_03,
                Self::L04 => STR_04,
                Self::L05 => STR_05,
                Self::L06 => STR_06,
                Self::L07 => STR_07,
                Self::L08 => STR_08,
                Self::L09 => STR_09,
                Self::L10 => STR_10,
            },
        )
    }
}
