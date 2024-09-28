### Pagen - PAssword GENerator

Simple program made while bored.

```terminal
pagen - simple password generator made in Rust

USAGE: pagen [OPTIONS]

OPTIONS:
     -u         Include uppercase symbols
     -n         Include numbers
     -l LENGTH  Specify the password length, default is 8
     -s         Include symbols
     -h, --help Show this help message
```

#### Using

- Compile it using cargo

```
cargo b --release
```

- The binary is `target/release/pagen`. Also you can run it or install it using cargo: See `cargo run -- --help` and `cargo install --path`
