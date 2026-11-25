use std::process::ExitCode;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--version") | Some("-V") => {
            println!("{NAME} {VERSION}");
            ExitCode::SUCCESS
        }
        Some("-h") | Some("--help") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(command) => {
            eprintln!("{NAME} {VERSION}: command `{command}` is not implemented yet");
            eprintln!("Run `{NAME} --help` for available flags");
            ExitCode::from(2)
        }
    }
}

fn print_help() {
    println!("{NAME} {VERSION}");
    println!("摩瓦 — 快速、可靠、可复现的软件生命周期管理器");
    println!();
    println!("Usage:");
    println!("  {NAME} --version        Print version");
    println!("  {NAME} --help           Print help");
    println!();
    println!("Version 0.0.0 is the bootstrap release. The lifecycle CLI is coming.");
}

#[cfg(test)]
mod tests {
    use super::{NAME, VERSION, print_help};

    #[test]
    fn metadata_is_consistent() {
        assert_eq!(NAME, "mova");
        assert_eq!(VERSION, "0.0.0");
    }

    #[test]
    fn help_runs_without_panic() {
        print_help();
    }
}
