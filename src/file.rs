use crate::cli;
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct WcStats {
    pub chars: usize,
    pub lines: usize,
    pub words: usize,
}

pub fn get_size_bytes(filename: &str) -> usize {
    let content = fs::read_to_string(filename).expect("file not found");
    content.len()
}

pub fn get_size_lines(filename: &str) -> usize {
    let content = fs::read_to_string(filename).expect("file not found");
    content.lines().count()
}

pub fn calc_sizes(cli_args: &cli::WcArgs) -> WcStats {
    let cli::WcMode {
        characters,
        lines,
        words,
    } = cli_args.mode;
    let mut all_sizes: WcStats = WcStats {
        chars: 0,
        lines: 0,
        words: 0,
    };

    let all_opts = characters || lines || words;

    if all_opts || characters {
        all_sizes.chars = get_size_bytes(&cli_args.file);
    }

    if all_opts || lines {
        all_sizes.lines = get_size_lines(&cli_args.file);
    }

    if all_opts || words {
        all_sizes.words = word_count(&cli_args.file) as usize;
    }
    all_sizes
}

// From https://stackoverflow.com/a/75644998
fn word_count(filename: &str) -> u32 {
    let s = fs::read_to_string(filename).expect("file not found");
    s.split_whitespace()
        .fold(HashMap::new(), |mut h, w| {
            *h.entry(w).or_insert(0) += 1;
            h
        })
        .values()
        .sum()
}
pub fn build_output_str(stats: &WcStats, mode: &cli::WcMode) -> String {
    let mut output: String = String::new();
    let all_opts = mode.characters || mode.lines || mode.words;

    if all_opts || mode.characters {
        let formatted = format!("{} ", stats.chars);
        output.push_str(&formatted);
    }

    if all_opts || mode.lines {
        let formatted = format!("{} ", stats.lines);
        output.push_str(&formatted);
    }

    if all_opts || mode.words {
        let formatted = format!("{} ", stats.words);
        output.push_str(&formatted);
    }
    output
}
