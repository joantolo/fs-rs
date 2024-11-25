use crate::error::{Error, Result};
use crate::query::Query;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Finder {
  query: Query,
  arc_matched_files: Arc<Mutex<Vec<String>>>,
}

impl Finder {
  pub fn new(query: Query) -> Finder {
    Finder {
      query,
      arc_matched_files: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub fn find(self) -> Result<Vec<String>> {
    self.matched_files_clear();

    self.recursive_find_from(&self.query.from)?;

    self.matched_files_move()
  }

  fn recursive_find_from(&self, dir: &PathBuf) -> Result<()> {
    let entries: Vec<PathBuf> = std::fs::read_dir(dir)?
      .filter_map(|entry| entry.ok())
      .map(|e| e.path())
      .collect();

    entries.into_par_iter().try_for_each(|path| {
      if path.is_dir() {
        self.recursive_find_from(&path)?;
      }

      if let Ok(matched) = self.query.matches(path) {
        self.matched_files_insert(matched);
      }

      Ok(())
    })
  }

  fn matched_files_insert(&self, matched: String) {
    self.arc_matched_files.lock().unwrap().push(matched);
  }

  fn matched_files_clear(&self) {
    self.arc_matched_files.lock().unwrap().clear();
  }

  fn matched_files_move(self) -> Result<Vec<String>> {
    let result =
      Arc::try_unwrap(self.arc_matched_files)
        .map_err(|_| Error::Arc("Has multiple users".to_string()))?
        .into_inner()?;

    Ok(result)
  }
}
