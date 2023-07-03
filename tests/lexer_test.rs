use markdown_rs::lexer::*;

mod utils;

#[test]
fn lexer_test_from_files() {
    pretty_env_logger::init();

    utils::read_test_data(|s| {
        let lexer = Lexer::new(s);

        for token in lexer {
            log::debug!("{:?}", token);
        }
    });
}
