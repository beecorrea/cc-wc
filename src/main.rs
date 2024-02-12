mod cli;
mod file;

fn main() {
    let args = cli::get_args();
    let stats = file::calc_sizes(&args);
    let output = file::build_output_str(&stats, &args.mode);
    println!("{}{}", output, args.file);
}
