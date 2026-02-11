use chrono::Local;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Level {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Level {
    pub fn from_env() -> Self {
        match std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".into())
            .to_lowercase()
            .as_str()
        {
            "error" => Level::Error,
            "warn" => Level::Warn,
            "debug" => Level::Debug,
            "trace" => Level::Trace,
            _ => Level::Info,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN ",
            Level::Info => "INFO ",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }
}

pub struct Logger {
    file: Mutex<std::fs::File>,
    path: PathBuf,
    level: Level,
}

impl Logger {
    pub fn new(out_dir: impl AsRef<Path>) -> io::Result<Self> {
        let out_dir = out_dir.as_ref();
        fs::create_dir_all(out_dir)?;

        let filename = format!("{}.log", Local::now().format("%Y%m%d_%H%M%S"));
        let path = out_dir.join(filename);

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        Ok(Self {
            file: Mutex::new(file),
            path,
            level: Level::from_env(),
        })
    }

    #[track_caller]
    fn log_inner(&self, level: Level, line: impl AsRef<str>) {
        if level > self.level {
            return;
        }

        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");

        let loc = std::panic::Location::caller();
        let file = loc.file();
        let line_no = loc.line();
        let short_file = file.rsplit('/').next().unwrap_or(file);

        let msg = format!(
            "[{}] [{}] [{}:{}] {}\n",
            ts,
            level.as_str(),
            short_file,
            line_no,
            line.as_ref()
        );

        print!("{}", msg);

        if let Ok(mut f) = self.file.lock() {
            let _ = f.write_all(msg.as_bytes());
            let _ = f.flush();
        }
    }

    // ---- public helpers ----

    #[track_caller]
    pub fn info(&self, msg: impl AsRef<str>) {
        self.log_inner(Level::Info, msg);
    }

    #[track_caller]
    pub fn debug(&self, msg: impl AsRef<str>) {
        self.log_inner(Level::Debug, msg);
    }

    #[track_caller]
    pub fn warn(&self, msg: impl AsRef<str>) {
        self.log_inner(Level::Warn, msg);
    }

    #[track_caller]
    pub fn error(&self, msg: impl AsRef<str>) {
        self.log_inner(Level::Error, msg);
    }

    #[track_caller]
    pub fn trace(&self, msg: impl AsRef<str>) {
        self.log_inner(Level::Trace, msg);
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
