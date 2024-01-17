use std::fs::{read_to_string, remove_file};
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write, ErrorKind};
use std::cmp::Ordering;
use std::path::Path;

const FILE_CHECK_INTERVAL: u64 = 5;

const MIN_BAR: f32 = 0.0;
const MAX_BAR: usize = 140;

const FILEPATH: &str = "/tmp/timer-rs_input";

// linearly interpolates A's position between B and C to D and E
fn lerp(a: f32, b: f32, c: f32, d: f32, e: f32) -> f32 {
    (a - b) * (e - d) / (c - b) + d
}

// add or remove characters from the right until len == max
fn adjust_len_right(mut msg: String, max: usize) -> String {
    let current_length = msg.chars().count();

    match current_length.cmp(&max) {
        Ordering::Less => {
            // Add spaces to the right side
            msg.push_str(&" ".repeat(max - current_length));
        }
        Ordering::Greater => {
            // Remove characters from the right side
            msg = msg.chars().take(max).collect();
        }
        Ordering::Equal => {}
    }

    msg
}

fn mk_bar(val: &f32, low: &f32, high: &f32, bar_low: &f32, bar_max: usize) -> String {
    let x = lerp(*val, *low, *high, *bar_low, bar_max as f32 - 1.0);
    let mut blocks: String = "█".repeat(x as usize);
    let y = x - x.trunc();
    let conversion = match y {
        x if (0.0..1.0 / 8.0).contains(&x) => " ",
        x if (1.0 / 8.0..2.0 / 8.0).contains(&x) => "▏",
        x if (2.0 / 8.0..3.0 / 8.0).contains(&x) => "▎",
        x if (3.0 / 8.0..4.0 / 8.0).contains(&x) => "▍",
        x if (4.0 / 8.0..5.0 / 8.0).contains(&x) => "▌",
        x if (5.0 / 8.0..6.0 / 8.0).contains(&x) => "▋",
        x if (6.0 / 8.0..7.0 / 8.0).contains(&x) => "▊",
        x if (7.0 / 8.0..1.0).contains(&x) => "▉",
        _ => "*",
    };
    blocks.push_str(conversion);
    let result = adjust_len_right(blocks, bar_max);
    result.to_string()
}

fn timer(seconds: f32) {
    let low: f32 = 0.0;
    let high: f32 = 8.0 * MAX_BAR as f32;
    let wait_interval: f32 = seconds / high;
    let sleep_dur = Duration::from_secs_f32(wait_interval);

    for i in ((low as u16)..(high as u16)).rev() {
        // println!("i: {}", i);
        let bar: String = mk_bar(&(i as f32), &low, &high, &MIN_BAR, MAX_BAR);
        println!("{}", bar);
        // io::stdout().flush().unwrap(); // Flush after printing
        sleep(sleep_dur)
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
