# Minigrep

Minigrep is a simple implementation of the `grep` command built in Rust. This project is part of my journey learning Rust.

## Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/sunRock98/minigrep-rust.git
cd minigrep
cargo build --release
```

## Usage

After building the project, you can use the minigrep command as follows:

```bash
cargo run --release 'searchquery' 'filename'
```

Replace 'searchquery' with the text you want to search for, and 'filename' with the name of the file you want to search in, like this:

example

```bash
cargo run --release 'To' 'poem.txt'
```

You can also set the `IGNORE_CASE` environment variable to enable case-insensitive search:

```bash
IGNORE_CASE=1 cargo run --release 'searchquery' 'filename'
```
