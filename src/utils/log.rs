use std::io::Write;
use std::sync::OnceLock;

use chrono::Local;

static INSTANCE: OnceLock<Log> = OnceLock::new();

pub struct Log;

impl Log {

    pub fn init() {
        INSTANCE.get_or_init(|| {
            if let Err(e) = std::fs::create_dir_all("logs") {
                eprintln!("Failed to create logs/ directory: {}", e);
            }
            Log
        });
    }

    pub fn info(message: &str) {
        Self::print(message, "INFO");
    }

    pub fn warn(message: &str) {
        Self::print(&format!("{} {} {}", Colors::FG_YELLOW, message, Colors::RESET), "WARN");
    }

    pub fn error(message: &str) {
        Self::print(&format!("{} {} {}", Colors::FG_RED, message, Colors::RESET), "ERROR");
    }

    pub fn debug(message: &str) {
        Self::print(&format!("{} {} {}", Colors::FG_BLUE, message, Colors::RESET), "DEBUG");
    }

    fn print(message: &str, level: &str) {
        let now = Local::now();
        let line = format!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), message);
        match level {
            "ERROR" | "WARN" => eprintln!("{}", line),
            _ => println!("{}", line),
        }
    }

    fn save(message: &str, print: bool) {
        let now = Local::now();
        let line = format!("[{}] {}\n", now.format("%Y-%m-%d %H:%M:%S").to_string(), message);
        let path = format!("logs/{}.log", now.format("%Y-%m-%d"));

        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
            let _ = file.write_all(line.as_bytes());
        }

        if print {
            Self::print(message, "ERROR");
        }
    }
}

struct Colors;

impl Colors {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BRIGHT: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const UNDERSCORE: &'static str = "\x1b[4m";
    pub const BLINK: &'static str = "\x1b[5m";
    pub const REVERSE: &'static str = "\x1b[7m";
    pub const HIDDEN: &'static str = "\x1b[8m";

    pub const FG_BLACK: &'static str = "\x1b[30m";
    pub const FG_RED: &'static str = "\x1b[31m";
    pub const FG_GREEN: &'static str = "\x1b[32m";
    pub const FG_YELLOW: &'static str = "\x1b[33m";
    pub const FG_BLUE: &'static str = "\x1b[34m";
    pub const FG_MAGENTA: &'static str = "\x1b[35m";
    pub const FG_CYAN: &'static str = "\x1b[36m";
    pub const FG_WHITE: &'static str = "\x1b[37m";

    pub const BG_BLACK: &'static str = "\x1b[40m";
    pub const BG_RED: &'static str = "\x1b[41m";
    pub const BG_GREEN: &'static str = "\x1b[42m";
    pub const BG_YELLOW: &'static str = "\x1b[43m";
    pub const BG_BLUE: &'static str = "\x1b[44m";
    pub const BG_MAGENTA: &'static str = "\x1b[45m";
    pub const BG_CYAN: &'static str = "\x1b[46m";
    pub const BG_WHITE: &'static str = "\x1b[47m";
}
