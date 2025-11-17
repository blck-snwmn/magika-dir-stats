# Magika Directory Stats (Python)

A CLI tool that analyzes files in a directory using [Magika](https://github.com/google/magika) and aggregates capacity by file type.

## Requirements

- Python 3.11+
- [uv](https://github.com/astral-sh/uv)

## Installation

```bash
uv sync
```

## Usage

```bash
# Analyze current directory
uv run python main.py

# Analyze specific directory
uv run python main.py /path/to/directory
```

## Output

The tool displays a table showing:
- File type (as detected by Magika)
- Total size per file type
- Percentage of total capacity

## Development

```bash
# Install development dependencies
uv sync --dev

# Run linter
uv run ruff check .

# Run formatter
uv run ruff format .
```
