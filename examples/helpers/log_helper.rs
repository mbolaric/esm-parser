use log::LevelFilter;
use log4rs::{
    Config,
    append::console::ConsoleAppender,
    config::{Appender, Root},
};

pub fn init_logging() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(
            log4rs::config::Logger::builder()
                .appender("stdout")
                .additive(false) // Prevents messages from propagating to the root logger
                .build("esm_parser", LevelFilter::Error), // Set esm_parser's level to Error
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}
