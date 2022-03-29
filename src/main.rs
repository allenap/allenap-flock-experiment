extern crate getch;
extern crate nix;

use nix::fcntl::{flock, FlockArg, FlockArg::*};
use std::fs::File;
use std::os::unix::io::AsRawFd;

fn main() {
    let mut file: Option<File> = None;

    println!(
        "Experiment with flock(2) on a file named `LOCK` in the current directory. Try:\n\n{}",
        HELP_KEY_BINDINGS
    );

    let getter = getch::Getch::new();
    loop {
        match getter.getch().unwrap() as char {
            'o' => {
                println!("Opening lock file...");
                match File::create("LOCK") {
                    Ok(f) => match file.replace(f) {
                        Some(_) => println!("-> Reopened lock file"),
                        None => println!("-> Opened lock file"),
                    },
                    Err(err) => eprintln!("-> FAILED: ${:?}", err),
                }
            }
            'c' => {
                println!("Closing lock file...");
                match file.take() {
                    Some(_) => println!("-> Closed lock file"),
                    None => println!("-> Lock file already closed"),
                }
            }
            's' => {
                println!("Obtaining shared lock...");
                lock(&file, LockShared, "Obtained shared lock.")
            }
            'S' => {
                println!("Obtaining shared lock... (non-blocking)");
                lock(&file, LockSharedNonblock, "Obtained shared lock.");
            }
            'x' => {
                println!("Obtaining exclusive lock...");
                lock(&file, LockExclusive, "Obtained exclusive lock.");
            }
            'X' => {
                println!("Obtaining exclusive lock... (non-blocking)");
                lock(&file, LockExclusiveNonblock, "Obtained exclusive lock.");
            }
            'u' => {
                println!("Unlocking...");
                lock(&file, Unlock, "Unlocked.");
            }
            'U' => {
                println!("Unlocking... (non-blocking)");
                lock(&file, UnlockNonblock, "Unlocked.");
            }
            'q' => {
                println!("Bye.");
                break;
            }
            'h' | '?' => {
                println!("Try:\n\n{}", HELP_KEY_BINDINGS);
            }
            chr => {
                eprintln!(
                    "Sorry, {:?} means nothing to me. Try:\n\n{}",
                    chr, HELP_KEY_BINDINGS
                );
            }
        }
    }
}

fn lock(file: &Option<File>, mode: FlockArg, message: &str) {
    match file {
        Some(f) => match flock(f.as_raw_fd(), mode) {
            Ok(_) => println!("-> {}", message),
            Err(err) => eprintln!("-> FAILED: ${:?}", err),
        },
        None => {
            eprintln!("-> FAILED: Lock file is not open")
        }
    };
}

static HELP_KEY_BINDINGS: &str = concat!(
    "  o – to open the lock file (do this first)\n",
    "  c – to close the lock file\n",
    "  s – to acquire a shared lock\n",
    "  S – to acquire a shared lock without blocking\n",
    "  x - to acquire an exclusive lock\n",
    "  X - to acquire an exclusive lock without blocking\n",
    "  u - to unlock\n",
    "  U - to unlock without blocking\n",
    "  q - to quit.\n",
    "  h or ? - for help.\n",
);
