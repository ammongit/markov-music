## markov-music
A music player that determines what song to next play based on a Markov chain. This way, based on how you normally listen to music, skipping songs you don't feel like hearing and repeating ones you liked, this program can 'learn' what you like and be able to generate new sequences of songs. The ideal is for it to generate an infinite playlist of songs that strike a balance between too random and too repetitive.

The music player's backend is based on `libmpv`.

Licensed under the GPL, version 2 or later.

### Compilation
Run `cargo build --release` in the top directory of the repository.

### Usage
```
USAGE:
    markov-music [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
        --no-color    Disables colors even on terminals that support them
    -V, --version     Prints version information

OPTIONS:
    -c, --config <FILE>    Use a specific configuration file instead of the
                           default
```

