use clap::Parser;

#[derive(Parser)]
#[command(name = "touchp")]
#[command(about = "Simple utility that combines `mkdir -p` and `touch` functionalities")]
#[command(version)]
pub struct Cli {
    /// Path of file to be created
    pub pathname: String,

    #[arg(short, long, help = "contents of the file")]
    pub contents: Option<String>,
}
