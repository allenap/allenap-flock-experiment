# allenap-flock-experiment

Simple interactive tool to experiment and learn about `flock` which is available
on [Linux](https://linux.die.net/man/2/flock), macOS, and other Unix-like
systems. It's behaviour can be somewhat subtle, and I found it helpful to get
hands-on experience with it when [writing about `flock` in 2017][flock-post].

[flock-post]: https://allenap.me/posts/flock-behaviour

## Command-line utility

If you have [installed Cargo][install-cargo], you can install
allenap-flock-experiment with `cargo install postgresfixture`. This puts a
`allenap-flock-experiment` binary in `~/.cargo/bin`, which the Cargo
installation process will probably have added to your `PATH`.

```shellsession
$ allenap-flock-experiment
Experiment with flock(2) on a file named `LOCK` in the current directory. Try:

  s – to acquire a shared lock
  S – to acquire a shared lock without blocking
  x - to acquire an exclusive lock
  X - to acquire an exclusive lock without blocking
  u - to unlock
  U - to unlock without blocking
  q - to quit.
  h or ? - for help.

Obtaining shared lock...
-> Obtained shared lock.
Obtaining exclusive lock...
-> Obtained exclusive lock.
Unlocking...
-> Unlocked.
Bye.
```

Above, I pressed the keys `s`, `x`, `u`, then `q` to exit.

That's all there is to it. However, it's more interesting to run it in a second
terminal at the same time, or even in a third. This should give you an idea of
how acquiring and releasing locks works in practice. Things to try:

- Compare the normal and the non-blocking modes.
- Run `allenap-flock-experiment` in three terminals at the same time, and
  acquire a shared lock in each. Then see what happens as you try to acquire an
  exclusive lock in each.
- Acquire an exclusive lock in one terminal, then try to acquire an exclusive
  lock in another terminal. In the first, switch to a shared lock. The
  documentation hints that the first may lose its hold on the lock altogether,
  but see what happens on your machine.
