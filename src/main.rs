use hathor::{FileGenerator, FileGeneratorBuilder};
use structopt::StructOpt;

mod cli;

use cli::Cli;

fn main() -> std::io::Result<()> {
    let cli = Cli::from_args();
    match cli {
        Cli::Generate {
            size,
            count,
            output_dir,
        } => FileGeneratorBuilder::with_size(size)
            .repeat(count)
            .generate_to(output_dir),
    }
}
