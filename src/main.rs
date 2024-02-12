mod cli;
mod file;

fn main() {
    let args = cli::get_args();
    let sizes = file::calc_sizes(&args);
    let output = file::build_output_str(&sizes, &args.mode);
    println!("{}{}", output, args.file);
}
