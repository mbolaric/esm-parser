// This file contains helper functions for interacting with the browser's console and setting up logging and panic hooks for the WebAssembly module.
use log::{self, Level, Log, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::console;

// A simple logger that redirects Rust's `log` macros to the browser's console.
struct WebConsoleLog;

// A global instance of the logger.
static LOG: WebConsoleLog = WebConsoleLog;

// Implementation of the `Log` trait for `WebConsoleLog`.
impl Log for WebConsoleLog {
    // Checks if a log message at a certain level should be logged.
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    // Logs a message to the browser's console.
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let msg = format!("[{}] {}", record.level(), record.args());
        match record.level() {
            Level::Error => console::error_1(&msg.into()),
            Level::Warn => console::warn_1(&msg.into()),
            Level::Info => console::info_1(&msg.into()),
            Level::Debug => console::log_1(&msg.into()),
            Level::Trace => console::debug_1(&msg.into()),
        }
    }

    // This logger doesn't buffer messages, so `flush` does nothing.
    fn flush(&self) {}
}

/// Initializes the console logger with a given log level.
/// This function is exposed to JavaScript and can be called to set up logging.
#[wasm_bindgen]
pub fn init_console_logging(log_level: LogLevel) -> Result<(), JsValue> {
    if log::set_logger(&LOG).is_ok() {
        let level: Level = log_level.to_native_level();
        log::set_max_level(level.to_level_filter());
        console::log_1(&format!("WASM logger initialized with Level: {:?}", log_level).into());
        Ok(())
    } else {
        // If the logger is already initialized, just update the max log level.
        let level: Level = log_level.to_native_level();
        log::set_max_level(level.to_level_filter());
        console::log_1(&format!("WASM logger reinitialized with Level: {:?}", log_level).into());
        Ok(())
    }
}

/// The log levels that can be set from JavaScript.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    // Converts the `LogLevel` enum to the corresponding `log::Level`.
    fn to_native_level(&self) -> Level {
        match self {
            LogLevel::Error => Level::Error,
            LogLevel::Warn => Level::Warn,
            LogLevel::Info => Level::Info,
            LogLevel::Debug => Level::Debug,
            LogLevel::Trace => Level::Trace,
        }
    }
}

// Bindings to JavaScript functions and objects.
#[wasm_bindgen]
extern "C" {
    // Binds to `console.error`.
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);

    // Binds to the JavaScript `Error` object.
    type Error;

    // Binds to the `Error` constructor to create a new error object.
    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    // Binds to the `stack` property of the `Error` object to get a stack trace.
    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}

/// The custom panic hook that is called when a panic occurs in the WASM module.
/// It logs the panic message and a JavaScript stack trace to the console.
pub fn hook(info: &panic::PanicHookInfo) {
    let mut msg = info.to_string();

    // Add a JavaScript stack trace to the panic message.
    msg.push_str("\n\nStack:\n\n");
    let e = Error::new();
    let stack = e.stack();
    msg.push_str(&stack);
    msg.push_str("\n\n");

    // Log the panic message to the console as an error.
    error(msg);
}

/// Sets the custom panic hook.
/// This function should be called once when the WASM module is initialized.
#[inline]
pub fn set_panic_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
        console::info_1(&"WASM Panic hook is set.".into());
    });
}
