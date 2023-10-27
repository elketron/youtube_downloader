use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub video: bool,
    #[clap(short, long)]
    pub music: bool,

    #[clap(short = 'l', long)]
    pub playlist: bool,

    #[clap(short, long)]
    pub path: Option<String>,

    #[clap(short = 'd', long)]
    pub mpv: bool,
}
