use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "butterlog", about = "Visualize and scan huge log files")]
struct Args {
    /// Path to the log file
    log_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.log_file.is_none() {
        eprintln!("missing log file argument");
        std::process::exit(1);
    }
}
