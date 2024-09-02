# Pather

Pather is a command-line utility written in Rust for managing the `PATH` environment variable. It allows you to easily add or remove directories from your `PATH`, with support for common shell configuration files such as `.bashrc`, `.zshrc`, and `config.fish`.

## Features

- Add a directory to the beginning of the `PATH` variable.
- Remove a directory from the `PATH` variable.
- Automatically update your shell configuration file.
- Supports `bash`, `zsh`, and `fish` shells.
- Command-line interface with options for displaying help and version information.

## Installation

To build and run Pather, you'll need to have [Rust](https://www.rust-lang.org/) installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and build the project:

```bash
git clone https://github.com/ptreew/pather.git
cd pather
cargo build --release
```

You can then run the binary:

```bash
./target/release/pather
```

## Usage

Pather can be used to add or remove directories from your `PATH` variable. If no arguments are provided, it will toggle the presence of the current directory in `PATH`.

### Add a Directory to `PATH`:

```bash
pather /path/to/directory
```

This will add `/path/to/directory` to the beginning of your `PATH` variable.

### Remove a Directory from `PATH`:

```bash
pather /path/to/directory
```

This will remove `/path/to/directory` from your `PATH` variable if it exists.

### Toggle Current Directory in `PATH`:

```bash
pather
```

Running Pather with no arguments will add the current directory to `PATH` if it is not already present, or remove it if it is.

### Display Help:

```bash
pather --help # -h can be used
```

### Display Version:

```bash
pather --version # -v can be used
```

### Example:

```bash
# Add /usr/local/bin to PATH
pather /usr/local/bin

# Remove /usr/local/bin from PATH
pather /usr/local/bin

# Add or remove the current directory from PATH
pather

# Display help message
pather --help

# Display version
pather --version

```

---

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on GitHub. Feel free to fork the repository.



## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
