use clap::Command;
use xtaskops::ops;

fn main() {
    let cli = Command::new("task")
        .subcommand(Command::new("coverage"))
        .subcommand(Command::new("dev"))
        .subcommand(Command::new("serve"));
    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("dev", _)) => {
            ops::cmd!(
                "cargo",
                "watch",
                "-s",
                "cd ./frontend && trunk build && cd ../",
                "-s",
                "cargo run -p backend"
            )
            .run()
            .unwrap();
        }
        Some(("serve", _)) => {
            ops::cmd!("trunk", "build", "--release")
                .dir("frontend")
                .run()
                .unwrap();
            ops::cmd!("cargo", "run", "-p", "backend", "--release")
                .run()
                .unwrap();
        }
        _ => unreachable!("unreachable branch"),
    };
}
