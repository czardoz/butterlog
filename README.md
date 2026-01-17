# butterlog

Butterlog is a fast CLI/TUI for scanning huge log files by grouping similar
prefixes into expandable partitions. It works with any log format and is built
for speed and navigation.

## Highlights
- Works on plain text logs with no required schema
- Partitions large files by prefix so you can drill down quickly
- Shows matching partitions and lines with background highlights
- Horizontal panning for long lines, automatic vertical scrolling
- Search mode with live input feedback

## How It Works
1. Reads the first 5000 lines to build a quick sample.
2. Estimates total lines using file size and average line length.
3. Targets roughly 2x the terminal height in top-level partitions.
4. Groups lines by prefix length to hit the target, then splits large groups.
5. Recursively partitions large groups with longer prefixes.
6. Renders partitions with a trailing "..." to indicate prefix grouping.

If the sample is small (less than 2x the screen height), Butterlog skips
partitioning and shows lines directly.

## Install

Build a release binary:

```bash
cargo build --release
```

Run the binary:

```bash
./target/release/butterlog <path/to/log>
```

## Usage

```bash
# Run the TUI
butterlog <path/to/log>

# Development run
cargo run -- <path/to/log>

# Run the pipeline without the TUI
butterlog --no-ui <path/to/log>
```

## Keybindings

| Key | Action |
| --- | --- |
| Up / Down | Move selection (auto scroll) |
| e | Expand partition |
| c | Collapse partition |
| Left / Right | Horizontal pan (4 columns) |
| / | Enter search mode |
| Enter | Apply search term |
| Esc | Cancel search input |
| q | Quit |

## Demo

```
> ERR...
  > WARN...
  v INFO...
    - 2024-05-01 10:10:01 INFO Boot sequence started
    - 2024-05-01 10:10:02 INFO Loaded module: network
    - 2024-05-01 10:10:03 INFO Listening on 0.0.0.0:8080

Search: timeout
```

## Search
- Press `/` to enter search mode.
- A prompt appears at the bottom: `Search: <your input>`.
- Search is case-insensitive and highlights matching partitions and lines.

## Examples

```bash
cargo run -- ./examples/android.log
cargo run -- ./examples/openstack_normal1.log
```

## Notes And Limitations
- Search and line viewing operate on the initial 5000-line sample.
- Partition counts are heuristic and based on the estimate, not exact totals.
