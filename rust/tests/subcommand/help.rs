use crate::subcommand::TestData;
use dnf_scaffold::command::subcommand::{
    help::NAME,
    Subcommand,
};
use std::sync::OnceLock;


pub(crate) fn test_data() -> &'static TestData {
    static INSTANCE: OnceLock<TestData> = OnceLock::new();
    INSTANCE.get_or_init(|| TestData {
        subcommand: Subcommand::Help,
        text: format!("{NAME}"),
    })
}
