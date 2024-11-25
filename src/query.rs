use crate::args::ParsedArgs;
use crate::error::{Result, Error};

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct Query {
  pub from: PathBuf,
  name: Option<String>,
}

impl Query {
  pub fn new(args: ParsedArgs) -> Result<Query> {
    let from = PathBuf::from(args.dir.unwrap_or_default());
    dir_exists(&from)?;

    Ok(Query{
      from,
      name: args.name,
    })
  }

  pub fn matches(&self, path: PathBuf) -> Result<String> {
    self.match_name(&path)?;

    Ok(path.into_os_string().into_string().unwrap())
  }

  fn match_name(&self, path: &Path) -> Result<()> {
    match &self.name {
      Some(name) => if Some(OsStr::new(name)) == path.file_name() {
        Ok(())
      } else {
        Err(Error::Other("Names don't match"))
      }
      None => Ok(())
    }
  }
}

fn dir_exists(dir: &PathBuf) -> Result<()> {
  std::fs::read_dir(dir)?;

  Ok(())
}
