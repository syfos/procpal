use thiserror::Error;
#[derive(Error, Debug)]
pub enum JobError {
  #[error("failed to parse command line: {0}")]
  LexError(String),

  #[error("failed to spawn process '{cmd}': {source}")]
  SpawnFailed {
    cmd: String,
    #[source]
    source: std::io::Error,
  },

  #[error("process exited with status {0}")]
  NonZeroExit(i32),
}
