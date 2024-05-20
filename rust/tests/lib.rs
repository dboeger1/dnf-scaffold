//mod options;
//mod subcommand;
//
//
//use dnf_scaffold::command::{
//    BIN_NAME,
//    Command,
//    options::CommandOptions,
//};
//use itertools::{
//    chain,
//    Itertools,
//};
//
//
//#[test]
//fn test_combinations() {
//    for count in chain(0..=4, 49..=52) {
//        for combination in Itertools::combinations(
//            options::test_data().iter(),
//            count,
//        ) {
//            // Set selected combination of command options.
//            let mut command_options = CommandOptions::default();
//            for command_option_info in combination.iter() {
//                command_options.set(&command_option_info.command_option);
//            }
//
//            // Verify options string.
//            let option_string = command_options.to_string();
//            let option_words: Vec<&str> = option_string
//                .split_whitespace()
//                .collect();
//            assert_eq!(combination.iter().count(), option_words.len());
//            for command_option_info in combination.iter() {
//                println!("{}", command_option_info.text);
//                assert!(option_words
//                    .iter()
//                    .any(|&word| {
//                        println!("word: \"{word}\"");
//                        println!("text: \"{}\"", command_option_info.text);
//                        word == command_option_info.text
//                    })
//                );
//            }
//
//            // Verify command string without subcommand.
//            let command_string = Command {
//                subcommand: None,
//                options: command_options.clone(),
//            }.to_string();
//            let expected: String;
//            if option_string.is_empty() {
//                expected = format!("{BIN_NAME}");
//            } else {
//                expected = format!("{BIN_NAME} {option_string}");
//            }
//            assert_eq!(expected, command_string);
//
//            // Verify command string with subcommand.
//            for test_data in subcommand::test_data() {
//                let subcommand_string = Command {
//                    subcommand: Some(test_data.subcommand.clone()),
//                    options: command_options.clone(),
//                }.to_string();
//                assert_eq!(
//                    format!("{expected} {}", test_data.text),
//                    subcommand_string,
//                );
//            }
//        }
//    }
//}
