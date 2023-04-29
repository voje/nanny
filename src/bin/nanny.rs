use clap::Parser;
use std::path::Path;
use std::fs::File;
use serde_yaml;
use chrono::{DateTime, Duration, Local};
use std::thread::sleep;

use nanny::shutdown::shutdown_with_message_wrapper;
use nanny::state::State;

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

    /// Tick frequency in seconds
    ///
    /// How often the program checks system time against its rules.
    #[arg(long, default_value_t = 60)]
    freq: u32,

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

fn main() {
    let args = Args::parse();

    if ! Path::new(args.state_path.as_str()).exists() {
        let writer = File::create(&args.state_path.as_str())
            .expect("Failed to create state file");

        let st = State::new(
			args.limit,
			args.start.as_str(),
			args.end.as_str(),
		);
        serde_yaml::to_writer(writer, &st)
            .expect("Failed to write init state");
    }

	let freq = Duration::seconds(args.freq.into());
	loop {
		// Read
		let fd = std::fs::File::open(&args.state_path)
			.expect("Failed opening state file");
		let mut st: State = serde_yaml::from_reader(fd)
			.expect("Failed parsing state file");

		// Action
    	println!("{:?}", st);
		let tn: DateTime<Local>  = Local::now();
		if ! st.tick(freq, tn) {
			println!("Tick failed, shutting down");
    		shutdown_with_message_wrapper("Enough computer for today.", 60, false).expect("Shutdown failed");

            /*
			println!("Waiting for the system to shut down");
			loop {
				sleep(Duration::minutes(1).to_std().unwrap());
			}
            */
		}

		// Write
		let fd = std::fs::File::create(&args.state_path)
			.expect("Failed opening state file");
        serde_yaml::to_writer(fd, &st)
            .expect("Failed writing state to file");

		// Wait
		sleep(freq.to_std().unwrap());
	}
}

