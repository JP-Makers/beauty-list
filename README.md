# beauty-list (bl) ✨

`beauty-list` (or simply `bl`) is a modern, colorful, and fast replacement for the standard `ls` command, written in Rust. It presents your files and directories in a clean, rounded table format with syntax highlighting and human-readable metadata.

## Features

- 🎨 **Colored Output**: Different colors for files, directories, and metadata.
- 📊 **Table Layout**: Organizes entries in a beautiful, rounded table.
- 📏 **Human-Readable Sizes**: Automatically converts byte counts to KB, MB, GB, etc.
- 🔒 **Permissions**: Displays Unix file modes (e.g., `drwxr-xr-x`) and octal values (e.g., `755`).
- ⏱️ **Timestamping**: Shows the last modified time in a clear format.
- 🦀 **Powered by Rust**: Fast, reliable, and memory-safe.

## Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then:

```bash
# Clone the repository
git clone https://gitlab.com/JP-Makers/beauty-list.git
cd beauty-list

# Build and install
cargo install --path .
```

After installation, you can use the command `bl`.

## Usage

List the contents of the current directory:

```bash
bl
```

List the contents of a specific directory:

```bash
bl /path/to/directory
```

### Help

Show version and help information:

```bash
bl --help
```

## Example Output

When you run `bl`, you'll see a beautiful output similar to this:

```text
╭────┬────────────┬──────┬──────────┬────────────┬───────┬─────────────────────╮
│ No │ Name       │ Type │ Size     │ Mode       │ Octal │ Modified            │
├────┼────────────┼──────┼──────────┼────────────┼───────┼─────────────────────┤
│ 1  │ src        │ dir  │ 4.0 KB   │ drwxr-xr-x │ 755   │ 2026-02-25 09:30:15 │
│ 2  │ Cargo.toml │ file │ 558 B    │ -rw-r--r-- │ 644   │ 2026-02-25 09:31:20 │
│ 3  │ README.md  │ file │ 2.1 KB   │ -rw-r--r-- │ 644   │ 2026-02-25 09:35:45 │
╰────┴────────────┴──────┴──────────┴────────────┴───────┴─────────────────────╯
```
*(Colors will vary depending on your terminal theme)*

## Dependencies

`beauty-list` stands on the shoulders of giants:
- `clap`: Command Line Argument Parser
- `tabled`: For pretty table formatting
- `owo-colors`: For terminal styling
- `chrono`: For time and date handling
- `strum`: For enum utilities

## License

This project is licensed under the [LICENSE](LICENSE) file in the repository.
