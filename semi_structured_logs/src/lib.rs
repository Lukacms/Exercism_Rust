//
// EPITECH PROJECT, 2022
// exercices_rust
// File description:
// lib
//

#![allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

fn log(level: LogLevel, message: &str) -> String {
    if level == LogLevel::Info {
        return info(message);
    }
    if level == LogLevel::Warning {
        return warn(message);
    }
    if level == LogLevel::Error {
        return error(message)
    }
    return message.to_string();
}

fn info(message: &str) -> String {
    let info_str: String = "[INFO]:".to_string();
    let result: String = format!("{} {}", info_str, message);

    return result;
}

fn warn(message: &str) -> String {
    let warn_str: String = "[WARNING]:".to_string();
    let result: String = format!("{} {}", warn_str, message);

    return result;
}

fn error(message: &str) -> String {
    let error_str: String = "[ERROR]:".to_string();
    let result: String = format!("{} {}", error_str, message);

    return result;
}

fn main() {
    let oui: String = log(LogLevel::Info, "Warn.");

    println!("{oui}");
}
