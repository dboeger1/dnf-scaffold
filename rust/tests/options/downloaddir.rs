use crate::options::TestData;
use dnf_scaffold::command::options::{
    CommandOption,
    downloaddir::{
        Arg,
        NAME,
    },
};
use std::sync::OnceLock;


pub(crate) fn test_data() -> &'static TestData {
    static INSTANCE: OnceLock<TestData> = OnceLock::new();
    INSTANCE.get_or_init(|| TestData {
        command_option: CommandOption::Downloaddir(Arg::default()),
        text: format!("--{NAME} {}", Arg::default().to_string_lossy()),
    })
}
