use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "butterlog", about = "Visualize and scan huge log files")]
struct Args {
    /// Path to the log file
    log_file: Option<String>,
}

fn main() {
    let _args = Args::parse();
}
