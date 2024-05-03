use std::fmt::Display;


pub const STR_ALL: &str = "all";
pub const STR_MAIN: &str = "main";
pub const STR_REPOID: &str = "repoid";


#[derive(Clone)]
pub enum Exclude {
    All,
    Main,
    RepoID(String),
}

impl Default for Exclude {
    fn default() -> Self {
        Self::All
    }
}

impl Display for Exclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => STR_ALL,
                Self::Main => STR_MAIN,
                Self::RepoID(repoid) => repoid,
            }
        )
    }
}
