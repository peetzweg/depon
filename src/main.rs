use std::{collections::HashMap, fs::File, io::Write, path::Path, time::SystemTime};

use clap::Parser;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Parser)]
struct Cli {
    observe: Vec<String>,
}

type DeponState = HashMap<String, u64>;

enum ExitCode {
    Do = 0isize,
    DoNot = 1isize,
}

fn main() {
    let args = Cli::parse();

    if args.observe.iter().count() == 0 {
        println!("No files to watch");
        std::process::exit(ExitCode::Do as i32);
    }

    let last_state = read_state();
    if last_state == None {
        println!("No lock file found");
    } else {
        println!("Lock file found");
    }

    let current_state: DeponState = args
        .observe
        .par_iter()
        .map(|file_path| {
            let modified_at = when_modified(&file_path);
            return (file_path.to_string(), modified_at);
        })
        .collect();

    let decision: ExitCode;
    if last_state.is_some() {
        if compare_state(current_state.clone(), last_state.unwrap()) {
            decision = ExitCode::Do;
        } else {
            decision = ExitCode::DoNot;
        }
    } else {
        decision = ExitCode::Do;
    }

    match decision {
        ExitCode::Do => {
            println!("Executing command");
            write_state(current_state.clone());
            std::process::exit(ExitCode::Do as i32);
        }
        ExitCode::DoNot => {
            println!("Skipping command");
            std::process::exit(ExitCode::DoNot as i32);
        }
    }
}

fn read_state() -> Option<DeponState> {
    const LOCK_FILE_PATH: &str = "./depon.lock";
    if Path::new(LOCK_FILE_PATH).exists() {
        let lock_file = File::open(LOCK_FILE_PATH);
        if lock_file.is_ok() {
            return Some(serde_yaml::from_reader(lock_file.unwrap()).unwrap());
        }
    }
    return None;
}

fn write_state(state: DeponState) {
    let yaml = serde_yaml::to_string(&state).expect("Unable to serialize state");

    const LOCK_FILE_PATH: &str = "./depon.lock";
    if Path::new(LOCK_FILE_PATH).exists() {
        std::fs::remove_file(LOCK_FILE_PATH).expect("Unable to remove file");
    }

    File::create(LOCK_FILE_PATH)
        .expect("Unable to create file")
        .write_all(yaml.as_bytes())
        .expect("Unable to write data");
}

fn when_modified(file_path: &String) -> u64 {
    let metadata = std::fs::metadata(&file_path)
        .expect(format!("Unable to read metadata of {}", file_path).as_str());

    let last_modified = metadata.modified().expect("modified time not available");
    return last_modified
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
}

fn compare_state(current_state: DeponState, last_state: DeponState) -> bool {
    if last_state.keys().count() != current_state.keys().count() {
        return true;
    }

    let result = current_state.iter().find(|f| {
        let current_file_path = f.0;
        let current_modified_at = f.1;
        let last_modified_at = last_state.get(current_file_path);
        if last_modified_at.is_some() {
            return last_modified_at.unwrap() != current_modified_at;
        }
        return true;
    });
    return result.is_some();
}
