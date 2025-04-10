use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command( name  = "rcli",  version, author, about, about  = None) ]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV,or other convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Number of times to greet
    #[arg(short,    long,   value_parser = verify_file_exist)]
    pub input: String,

    /// Number of times to greet
    #[arg(short, long, default_value = "output.json")] //"output.json".into()
    pub output: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = '.')]
    pub delimiter: char,

    /// Number of times to greet
    #[arg(long, default_value_t = true)]
    pub header: bool,
}
fn verify_file_exist(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File doesn't exist")
    }
}
