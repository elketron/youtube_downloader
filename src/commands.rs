use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub video: bool,
    #[clap(short, long)]
    pub music: bool,
}
