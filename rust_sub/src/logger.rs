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

        let filename = format!("{}.log", Local::now().format("%Y%m%d%H%M"));
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

    pub fn log(&self, line: impl AsRef<str>) {
        // Best-effort logging: don't panic if file write fails.
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        let msg = format!("[{}] {}\n", ts, line.as_ref());

        // stdout too (handy for docker compose logs)
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
