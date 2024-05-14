use crate::options::TestData;
use dnf_scaffold::command::options::{
    CommandOption,
    noplugins::NAME,
};
use std::sync::OnceLock;


pub(crate) fn test_data() -> &'static TestData {
    static INSTANCE: OnceLock<TestData> = OnceLock::new();
    INSTANCE.get_or_init(|| TestData {
        command_option: CommandOption::Noplugins,
        text: format!("--{NAME}"),
    })
}
