use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct WcArgs {
    file: String,

    #[arg(short, long)]
    characters: bool,
}

pub fn get_args() -> WcArgs {
    return WcArgs::parse();
}
