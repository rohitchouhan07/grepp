use clap::Parser;
use std::process;
use grepp::CliArgs;

fn parse_args() -> CliArgs {
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[arg(long, short)]
        file_path: String,
        #[arg(long, short)]
        pattern: String,
    }
    let args: Args = Args::parse();

    CliArgs {
        file_path: args.file_path,
        pattern: args.pattern,
    }
}

fn main() {
    let cli_args: CliArgs = parse_args();

    if let Err(e) = grepp::run(cli_args) {
        println!("Error: {e}");
        process::exit(1);
    }
}
