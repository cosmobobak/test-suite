use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
#[allow(clippy::struct_excessive_bools, clippy::option_option)]
pub struct Cli {
    #[clap(value_name = "ENGINE")]
    /// The path to the engine executable.
    pub engine: std::path::PathBuf,
    /// The path to an Extended Position Description file to run as a test suite.
    #[clap(value_name = "EPD FILE")]
    pub epdpath: std::path::PathBuf,
    /// Time in milliseconds to search for each move when doing a test suite.
    #[clap(long, value_name = "MS", default_value = "3000")]
    pub time: u64,
    /// Whether to print the program's thinking to stdout when doing an epd test suite.
    #[clap(long)]
    pub verbose: bool,
    /// UCI options to pass to the engine.
    #[clap(long, value_name = "NAME=VALUE")]
    pub options: Vec<(String, Option<String>)>,
}
