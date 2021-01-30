use std::path::PathBuf;

use structopt::StructOpt;

/// Hathor - a file generator
#[derive(Debug, StructOpt)]
pub enum Cli {
    /// Generates a set of same size files with repeating pattern `0123456789`.
    Generate {
        /// Size of each file.
        size: usize,
        /// Number of files to create.
        count: usize,
        /// Directory to put generated files to.
        output_dir: PathBuf,
    },
}
