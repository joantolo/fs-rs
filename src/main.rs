mod args;
mod error;
mod find;
mod query;

use crate::args::ParsedArgs;
use crate::error::Result;
use crate::find::Finder;
use crate::query::Query;

fn main() -> Result<()> {
    let args = ParsedArgs::get_parsed();
    let query = Query::new(args)?;
    let finder = Finder::new(query);

    let result = finder.find()?;

    result.iter().for_each(|found| println!("{}", found));

    Ok(())
}
