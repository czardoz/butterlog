use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "butterlog", about = "Visualize and scan huge log files")]
struct Args {
    /// Path to the log file
    log_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let Some(log_file) = args.log_file else {
        eprintln!("missing log file argument");
        std::process::exit(1);
    };

    let path = std::path::Path::new(&log_file);
    match std::fs::metadata(path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                eprintln!("path is not a file");
                std::process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("log file not found");
            std::process::exit(1);
        }
    }
}
