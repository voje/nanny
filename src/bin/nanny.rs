use nanny::shutdown::shutdown_with_message_wrapper;
use clap::Parser;
use serde::{Serialize, Deserialize};
use serde_yaml;
use std::path::Path;
use std::fs::File;
use std::time::{SystemTime, Duration};

#[derive(Parser, Debug)]
struct Args {
    /*
    /// Path to config file
    #[arg(short, long)]
    config: String 
    */

    /// Daily limit in minutes
    #[arg(long)]
    limit: u32,

    /// Start time hh:mm
    #[arg(long)]
    start: String,

    /// End time hh:mm
    #[arg(long)]
    end: String,

    /// State file path
    ///
    /// This program stores is state in a .yaml file. Hide it well.
    #[arg(long)]
    state_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    // TODO represent time
    last_sync: SystemTime,
    daily_limit_left: Duration,
}

fn init_state(args: &Args) -> State {
    State {
        last_sync: SystemTime::now(),
        daily_limit_left: Duration::new(args.limit.into(), 0),
    }
}

fn main() {
    let args = Args::parse();

    if ! Path::new(args.state_path.as_str()).exists() {
        let writer = File::create(args.state_path.as_str())
            .expect("Failed to create state file");
        let is = init_state(&args);
        serde_yaml::to_writer(writer, &is)
            .expect("Failed to write init state");
    }

    let reader = std::fs::File::open(args.state_path)
        .expect("Failed opening state file");
    let state: State = serde_yaml::from_reader(reader)
        .expect("Failed parsing state file");

    println!("{:?}", state);

    shutdown_with_message_wrapper("Test123", 60, false).expect("Shutdown failed");
}
