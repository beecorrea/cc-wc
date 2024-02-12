use crate::cli;
use std::fs;

pub struct WcSizes {
    pub chars: usize,
    pub lines: usize,
}

pub fn get_size_bytes(filename: &str) -> usize {
    let content = fs::read_to_string(filename).expect("file not found");
    content.len()
}

pub fn get_size_lines(filename: &str) -> usize {
    let content = fs::read_to_string(filename).expect("file not found");
    content.lines().count()
}

pub fn calc_sizes(cli_args: &cli::WcArgs) -> WcSizes {
    let cli::WcMode { characters, lines } = cli_args.mode;
    let mut all_sizes: WcSizes = WcSizes { chars: 0, lines: 0 };

    if characters {
        all_sizes.chars = get_size_bytes(&cli_args.file)
    }

    if lines {
        all_sizes.lines = get_size_lines(&cli_args.file)
    }
    all_sizes
}

pub fn build_output_str(sizes: &WcSizes, mode: &cli::WcMode) -> String {
    let mut output: String = String::new();

    if mode.characters {
        let formatted = format!("{} ", sizes.chars);
        output.push_str(&formatted);
    }

    if mode.lines {
        let formatted = format!("{} ", sizes.lines);
        output.push_str(&formatted);
    }

    output
}
