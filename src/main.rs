extern crate getch;
extern crate nix;

use nix::fcntl::{flock, FlockArg};
use std::fs::File;
use std::os::unix::io::AsRawFd;

fn report(result: nix::Result<()>, message: &str) {
    match result {
        Ok(_) => println!("-> {}", message),
        Err(err) => eprintln!("FAILED: ${:?}", err),
    }
}

fn main() {
    let file = File::create("LOCK").unwrap();
    let fd = file.as_raw_fd();
    let getter = getch::Getch::new();
    println!(
        "Experiment with flock(2) on a file named `LOCK` in the current directory. Try:\n\n{}",
        HELP_KEY_BINDINGS
    );
    loop {
        match getter.getch().unwrap() as char {
            's' => {
                println!("Obtaining shared lock...");
                report(flock(fd, FlockArg::LockShared), "Obtained shared lock.")
            }
            'S' => {
                println!("Obtaining shared lock... (non-blocking)");
                report(
                    flock(fd, FlockArg::LockSharedNonblock),
                    "Obtained shared lock.",
                );
            }
            'x' => {
                println!("Obtaining exclusive lock...");
                report(
                    flock(fd, FlockArg::LockExclusive),
                    "Obtained exclusive lock.",
                );
            }
            'X' => {
                println!("Obtaining exclusive lock... (non-blocking)");
                report(
                    flock(fd, FlockArg::LockExclusiveNonblock),
                    "Obtained exclusive lock.",
                );
            }
            'u' => {
                println!("Unlocking...");
                report(flock(fd, FlockArg::Unlock), "Unlocked.");
            }
            'U' => {
                println!("Unlocking... (non-blocking)");
                report(flock(fd, FlockArg::UnlockNonblock), "Unlocked.");
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
    "  u - to unlock without blocking\n",
    "  q - to quit.\n",
    "  h or ? - for help.\n",
);
