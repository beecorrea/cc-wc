use clap::Args;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct WcArgs {
    pub file: String,

    #[command(flatten)]
    pub mode: WcMode,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct WcMode {
    #[arg(short)]
    pub characters: bool,

    #[arg(short)]
    pub lines: bool,

    #[arg(short)]
    pub words: bool,
}

pub fn get_args() -> WcArgs {
    WcArgs::parse()
}

pub fn use_all_opts(mode: &WcMode) -> bool {
    !(mode.characters || mode.lines || mode.words)
}
