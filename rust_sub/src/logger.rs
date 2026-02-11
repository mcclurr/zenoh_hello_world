use chrono::Local;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};

pub struct Logger {
    file: Mutex<std::fs::File>,
    path: PathBuf,
}

impl Logger {
    /// Creates out_dir if missing, then creates a file like out/202502111347.log
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
        })
    }

    /// Core log function with caller info
    #[track_caller]
    pub fn log(&self, line: impl AsRef<str>) {
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");

        let loc = std::panic::Location::caller();
        let file = loc.file();
        let line_no = loc.line();

        // Just the filename instead of full path
        let short_file = file.rsplit('/').next().unwrap_or(file);

        let msg = format!(
            "[{}] [{}:{}] {}\n",
            ts,
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

    pub fn path(&self) -> &Path {
        &self.path
    }
}
