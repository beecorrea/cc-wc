use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct WcArgs {
    pub file: String,

    #[arg(short, long)]
    pub characters: bool,
}

pub fn get_args() -> WcArgs {
    WcArgs::parse()
}
