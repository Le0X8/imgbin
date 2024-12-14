use clap::{Parser, Subcommand};
use imgbin::{b2i, i2b, i2d};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Binary2Image
    B {
        path: String,
        #[clap(long, short, action)]
        noalpha: bool,
        #[clap(long, short, action)]
        grayscale: bool,
    },

    /// Image2Binary
    I { path: String },

    /// Image2Data
    D { path: String },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::B {
            path,
            noalpha,
            grayscale,
        } => {
            b2i(path, !noalpha, *grayscale);
        }
        Commands::I { path } => {
            i2b(path);
        }
        Commands::D { path } => {
            i2d(path);
        }
    }
}
