# README

### Install Rust

Install rustup:
```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Check Rust version:
```shell
rustc --version
```

### Updating and Uninstalling Rust

Update Rust:
```shell
rustup update
```

Uninstall Rust:
```shell
rustup self uninstall
```

### Rust Documentation

View rust documentation locally:
```shell
rustup doc
```

Or visit https://doc.rust-lang.org/

### Cargo project

The guessing_game example from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

```shell
# create a new cargo project
cargo new guessing_game

# change into the newly created project directory
cd guessing_game

# build the project
# creates the project executable file within the ./target/debug folder
cargo build

# run the project
cargo run

# check the project compiles
# faster alternative to rebuilding each time
cargo check

# create an optimised release build
# benchmark against this version of the project
cargo build --release
```
