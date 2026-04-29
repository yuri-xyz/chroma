pub mod audio_fixtures;
pub mod audio_trace;

use std::path::PathBuf;

#[allow(dead_code)]
pub fn fresh_test_dir(test_name: &str) -> PathBuf {
  let path = std::env::temp_dir().join("chroma-tests").join(test_name);

  if path.exists() {
    std::fs::remove_dir_all(&path).expect("failed to clean existing test directory");
  }

  std::fs::create_dir_all(&path).expect("failed to create test directory");

  path
}
