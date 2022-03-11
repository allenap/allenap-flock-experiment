extern crate getch;
extern crate nix;

use nix::fcntl::{flock, FlockArg};
use std::fs::File;
use std::os::unix::io::AsRawFd;

fn main() {
    let file = File::create("LOCK").unwrap();
    let fd = file.as_raw_fd();
    let lock = |mode: FlockArg, message: &str| match flock(fd, mode) {
        Ok(_) => println!("-> {}", message),
        Err(err) => eprintln!("-> FAILED: ${:?}", err),
    };

    println!(
        "Experiment with flock(2) on a file named `LOCK` in the current directory. Try:\n\n{}",
        HELP_KEY_BINDINGS
    );

    let getter = getch::Getch::new();
    loop {
        match getter.getch().unwrap() as char {
            's' => {
                println!("Obtaining shared lock...");
                lock(FlockArg::LockShared, "Obtained shared lock.")
            }
            'S' => {
                println!("Obtaining shared lock... (non-blocking)");
                lock(FlockArg::LockSharedNonblock, "Obtained shared lock.");
            }
            'x' => {
                println!("Obtaining exclusive lock...");
                lock(FlockArg::LockExclusive, "Obtained exclusive lock.");
            }
            'X' => {
                println!("Obtaining exclusive lock... (non-blocking)");
                lock(FlockArg::LockExclusiveNonblock, "Obtained exclusive lock.");
            }
            'u' => {
                println!("Unlocking...");
                lock(FlockArg::Unlock, "Unlocked.");
            }
            'U' => {
                println!("Unlocking... (non-blocking)");
                lock(FlockArg::UnlockNonblock, "Unlocked.");
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

static HELP_KEY_BINDINGS: &str = concat!(
    "  s – to acquire a shared lock\n",
    "  S – to acquire a shared lock without blocking\n",
    "  x - to acquire an exclusive lock\n",
    "  X - to acquire an exclusive lock without blocking\n",
    "  u - to unlock\n",
    "  U - to unlock without blocking\n",
    "  q - to quit.\n",
    "  h or ? - for help.\n",
);
