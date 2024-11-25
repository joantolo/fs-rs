pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct ParsedArgs {
    #[arg(value_name = "DIRECTORY", default_value = ".")]
    pub dir: Option<String>,

    #[arg(short, long)]
    pub name: Option<String>,
}

impl ParsedArgs {
    pub fn get_parsed() -> Self {
        Self::parse()
    }
}
