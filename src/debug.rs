use std::{
  fs::{File, OpenOptions},
  io::{self, BufWriter, Sink, Write},
  path::Path,
};

pub const DEFAULT_DEBUG_LOG_PATH: &str = "debug.log";

pub enum DebugLog {
  File(BufWriter<File>),
  Sink(BufWriter<Sink>),
}

impl DebugLog {
  pub fn create_default() -> io::Result<Self> {
    Self::create_at(DEFAULT_DEBUG_LOG_PATH)
  }

  pub fn create_at<P: AsRef<Path>>(path: P) -> io::Result<Self> {
    if debug_logging_enabled() {
      return Self::file(path);
    }

    Ok(Self::sink())
  }

  pub fn file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
    File::create(path.as_ref())?;
    let file = OpenOptions::new().create(true).append(true).open(path)?;

    Ok(Self::File(BufWriter::new(file)))
  }

  pub fn sink() -> Self {
    Self::Sink(BufWriter::new(io::sink()))
  }
}

impl Write for DebugLog {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match self {
      Self::File(writer) => writer.write(buf),
      Self::Sink(writer) => writer.write(buf),
    }
  }

  fn flush(&mut self) -> io::Result<()> {
    match self {
      Self::File(writer) => writer.flush(),
      Self::Sink(writer) => writer.flush(),
    }
  }
}

pub fn debug_logging_enabled() -> bool {
  cfg!(debug_assertions)
}

pub fn frame_logging_enabled() -> bool {
  debug_logging_enabled()
    && std::env::var("CHROMA_TRACE_FRAMES")
      .is_ok_and(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

pub fn append_debug_line(component: &str, message: impl AsRef<str>) {
  if !debug_logging_enabled() {
    return;
  }

  if let Ok(mut file) = OpenOptions::new()
    .create(true)
    .append(true)
    .open(DEFAULT_DEBUG_LOG_PATH)
  {
    let _ = writeln!(file, "[{component}] {}", message.as_ref());
  }
}
