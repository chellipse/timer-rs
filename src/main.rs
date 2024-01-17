use std::fs::{read_to_string, remove_file};
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, ErrorKind};
// use std::io::{self, Write, ErrorKind};
use std::path::Path;
// use std::cmp::Ordering;

const FILE_CHECK_INTERVAL: u64 = 2;

// const MIN_BAR: f32 = 0.0;
const MAX_BAR: usize = 140;

const FILEPATH: &str = "/tmp/timer-rs_input";

const SLIVERS: [char; 8] = ['▉','▊','▋','▌','▍','▎','▏',' '];

fn timer(seconds: f32) {
    let high: f32 = 8.0 * MAX_BAR as f32;
    let wait_interval: f32 = seconds / high;
    let sleep_dur = Duration::from_secs_f32(wait_interval);

    let mut bar: String = "█".repeat(MAX_BAR);
    for i in (0..(bar.chars().count())).rev() {
        for char in SLIVERS {
            let mut chars: Vec<char> = bar.chars().collect();

            chars[i] = char;

            bar = String::with_capacity(chars.len());
            for c in chars {
                bar.push(c);
            };

            println!("{}", bar);
            sleep(sleep_dur)
        }
    }
}

fn try_file_as_u32() -> io::Result<u32>  {
    match read_to_string(FILEPATH) {
        Ok(contents) => match contents.trim().parse::<u32>() {
            Ok(number) => Ok(number),
            Err(_) => Err(io::Error::new(ErrorKind::InvalidData, "Failed to parse file contents")),
        },
        Err(e) => Err(e),
    }
}

fn main() {
    // println!("FILEPATH: {}", FILEPATH);
    loop {
        if Path::new(FILEPATH).exists() {
            // println!("File exists: {}", FILEPATH);
            match try_file_as_u32() {
                Ok(x) => {
                    // println!("Start");
                    timer(x as f32);
                    // println!("End");
                    match remove_file(FILEPATH) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Remove file err: {}", e);
                            std::process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    println!("Err: {}", e);
                    continue
                }
            };

            sleep(Duration::from_secs(FILE_CHECK_INTERVAL));
        }
        else {
            // println!("File does not exist: {}", FILEPATH);
            println!("[IDLE]");
        }
        sleep(Duration::from_secs(FILE_CHECK_INTERVAL));
    }
}
