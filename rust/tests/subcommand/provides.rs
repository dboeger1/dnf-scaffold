use crate::subcommand::TestData;
use dnf_scaffold::command::subcommand::{
    provides::NAME,
    Subcommand,
};
use std::sync::OnceLock;


pub(crate) fn test_data() -> &'static TestData {
    static INSTANCE: OnceLock<TestData> = OnceLock::new();
    INSTANCE.get_or_init(|| TestData {
        subcommand: Subcommand::Provides,
        text: format!("{NAME}"),
    })
}
