extern crate getch;
extern crate nix;

use std::fs::File;
use std::os::unix::io::AsRawFd;
use nix::fcntl::{flock,FlockArg};


fn main() {
    let file = File::create("LOCK").unwrap();
    let fd = file.as_raw_fd();
    let getter = getch::Getch::new();
    loop {
        match getter.getch().unwrap() as char {
            's' => {
                println!("Trying to obtain shared lock...");
                flock(fd, FlockArg::LockShared).unwrap();
                println!("Obtained shared lock.");
            },
            'x' => {
                println!("Trying to lock exclusive...");
                flock(fd, FlockArg::LockExclusive).unwrap();
                println!("Obtained exclusive lock.");
            },
            'u' => {
                println!("Trying to unlock...");
                flock(fd, FlockArg::Unlock).unwrap();
                println!("Unlocked.");
            },
            'q' => {
                println!("Bye.");
                break;
            },
            chr => {
                println!(concat!(
                    "Sorry, {:?} means nothing to me. Try s to acquire a ",
                    "shared lock, x to acquire an exclusive lock, u to ",
                    "unlock, or q to quit."), chr);
            },
        }
    }
}
