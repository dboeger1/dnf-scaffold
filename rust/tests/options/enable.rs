use crate::options::TestData;
use dnf_scaffold::command::options::{
    CommandOption,
    enable::NAME,
};
use std::sync::OnceLock;


pub(crate) fn test_data() -> &'static TestData {
    static INSTANCE: OnceLock<TestData> = OnceLock::new();
    INSTANCE.get_or_init(|| TestData {
        command_option: CommandOption::Enable,
        text: format!("--{NAME}"),
    })
}