use clap::Parser;

/// Tool for creating and reading hidden messages inside PNG files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// command to execute on the given png
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// hides a given message inside a png file and writes it to the desired destination
    Hide {
        /// path to the source image
        #[clap(short, long)]
        input_path: String,

        /// string representation of a PNG chunk type
        #[clap(short, long)]
        chunk_type: String,

        /// the message that should be hidden in the src image
        #[clap(short, long)]
        message: String,

        // path where the resulted image should be
        #[clap(short, long)]
        output_path: Option<String>,
    },
    /// decodes and prints the message(s) inside the given png file
    Find {
        /// path to the source image
        #[clap(short, long)]
        path: String,

        /// string representation of a PNG chunk type
        #[clap(short, long)]
        chunk_type: String,
    },
    /// removes a given message from the given png file
    Delete {
        /// path to the source image
        #[clap(short, long)]
        path: String,

        /// string representation of a PNG chunk type
        #[clap(short, long)]
        chunk_type: String,
    },
}
