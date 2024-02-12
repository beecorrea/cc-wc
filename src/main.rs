mod cli;
mod file;

fn main() {
    let args = cli::get_args();
    let size = file::get_size_bytes(&args.file);
    println!("{} {:}", size, args.file);
}
