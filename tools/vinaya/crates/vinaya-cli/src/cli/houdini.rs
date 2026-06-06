mod generate_proto;
mod hfs;
mod latest;
mod list;
use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    GenerateProto(generate_proto::Args),
    Hfs(hfs::Args),
    Latest(latest::Args),
    List(list::Args),
}
pub fn execute(args: Args) -> mischief::Result<()> {
    match args.command {
        Commands::GenerateProto(cmd) => generate_proto::execute(cmd),
        Commands::Hfs(cmd) => hfs::execute(cmd),
        Commands::Latest(cmd) => latest::execute(cmd),
        Commands::List(cmd) => list::execute(cmd),
    }
}
