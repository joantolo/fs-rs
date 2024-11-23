use crate::args::ParsedArgs;
use crate::error::Result;

use std::ffi::OsStr;
use std::path::PathBuf;

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

  pub fn matches(&self, path: PathBuf) -> Option<String> {
    self.match_name(&path)
      .then(|| path.into_os_string().into_string().unwrap())
  }

  fn match_name(&self, path: &PathBuf) -> bool {
    match &self.name {
      Some(name) => Some(OsStr::new(name)) == path.file_name(),
      None => true,
    }
  }
}

fn dir_exists(dir: &PathBuf) -> Result<()> {
  std::fs::read_dir(dir)?;

  Ok(())
}
