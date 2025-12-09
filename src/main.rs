mod cmd;
mod util;

use clap::Parser;
use cmd::cli::Cli;
use util::{
    create_file::create_file,
    resolve_path::resolve_path,
};

fn main() {
    let cli = Cli::parse();
    let absolute_file_path_to_create = resolve_path(&cli.pathname);
    match absolute_file_path_to_create {
        Err(error) => {
            let description = error.get_description();
            eprintln!("{}", description);
        }
        Ok(absolute_file_path) => {
            if let Err(error) = create_file(absolute_file_path, cli.contents) {
                let description = error.get_description();
                eprintln!("{}", description);
            }
        }
    }
}
