# FindR

A fast and memory-efficient command-line tool to search for patterns in files, written in Rust.

## Features

- Memory-efficient file processing using buffered reading
- Simple and intuitive command-line interface
- Fast pattern matching
- Handles files of any size

## Installation

### From Crates.io

```bash
cargo install martinrepo-findr
```

### From Source

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs)
2. Clone this repository:
   ```bash
   git clone https://github.com/MartinRepo/FindR.git
   cd findr
   ```
3. Build and install:
   ```bash
   cargo install --path .
   ```

## Usage

```bash
findr <pattern> <path>
```

### Arguments

- `pattern`: The text pattern to search for
- `path`: Path to the file to search in

### Examples

Search for "hello" in a file:
```bash
findr hello file.txt
```

Search for "error" in a log file:
```bash
findr error server.log
```

## How it Works

FindR uses buffered reading to process files line by line, making it memory efficient even for very large files. It reads the input file in chunks rather than loading the entire file into memory at once.

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running in Debug Mode

```bash
cargo run -- <pattern> <path>
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.