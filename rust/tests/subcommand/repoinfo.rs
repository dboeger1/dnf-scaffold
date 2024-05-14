use crate::subcommand::TestData;
use dnf_scaffold::command::subcommand::{
    repoinfo::NAME,
    Subcommand,
};
use std::sync::OnceLock;


pub(crate) fn test_data() -> &'static TestData {
    static INSTANCE: OnceLock<TestData> = OnceLock::new();
    INSTANCE.get_or_init(|| TestData {
        subcommand: Subcommand::Repoinfo,
        text: format!("{NAME}"),
    })
}