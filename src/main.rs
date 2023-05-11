use clap::Parser;

mod cli;
mod runner;

fn main() {
    let cli = cli::Cli::parse();
    let exe = cli.engine;
    let epdpath = cli.epdpath;
    let time = cli.time;
    let hash = cli.hash;
    let threads = cli.threads;
    let verbose = cli.verbose;

    let mut proc = std::process::Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn engine process");

    proc.kill().unwrap();
}
