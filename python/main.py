#!/usr/bin/env python3
"""
Magika Directory Stats CLI
Analyzes files in a directory using Magika and aggregates capacity by file type
"""

import argparse
import os
import sys
from collections import defaultdict
from pathlib import Path

from magika import Magika


def scan_directory(directory: Path, magika: Magika) -> dict[str, int]:
    """
    Recursively scans a directory and aggregates capacity by file type

    Args:
        directory: The directory to scan
        magika: Magika instance

    Returns:
        Mapping of file type -> total bytes
    """
    file_type_sizes = defaultdict(int)

    for root, _, files in os.walk(directory):
        for filename in files:
            filepath = Path(root) / filename

            # Skip symbolic links
            if filepath.is_symlink():
                continue

            try:
                # Get file size
                file_size = filepath.stat().st_size

                # Identify file type with Magika
                result = magika.identify_path(filepath)
                file_type = result.output.label

                # Aggregate
                file_type_sizes[file_type] += file_size

            except (OSError, PermissionError) as e:
                # Skip files that cannot be accessed
                print(f"Warning: Could not process {filepath}: {e}", file=sys.stderr)
                continue

    return dict(file_type_sizes)


def format_size(size_bytes: int) -> str:
    """
    Converts bytes to human-readable format

    Args:
        size_bytes: Number of bytes

    Returns:
        Formatted string (e.g., "1.5 MB")
    """
    for unit in ["B", "KB", "MB", "GB", "TB"]:
        if size_bytes < 1024.0:
            return f"{size_bytes:.2f} {unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.2f} PB"


def display_results(file_type_sizes: dict[str, int]) -> None:
    """
    Displays aggregation results (capacity + percentage)

    Args:
        file_type_sizes: Mapping of file type -> total bytes
    """
    if not file_type_sizes:
        print("No files found.")
        return

    # Calculate total size
    total_size = sum(file_type_sizes.values())

    # Header
    print("\n" + "=" * 70)
    print(f"{'File Type':<30} {'Size':<20} {'Percentage':>10}")
    print("=" * 70)

    # Sort file types by size in descending order
    sorted_items = sorted(file_type_sizes.items(), key=lambda x: x[1], reverse=True)

    for file_type, size in sorted_items:
        percentage = (size / total_size) * 100
        formatted_size = format_size(size)
        print(f"{file_type:<30} {formatted_size:<20} {percentage:>9.2f}%")

    # Total
    print("=" * 70)
    print(f"{'Total':<30} {format_size(total_size):<20} {'100.00%':>10}")
    print("=" * 70 + "\n")


def main():
    """
    Analyzes files in the specified directory using Magika
    and displays capacity aggregated by file type.
    """
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("directory", nargs="?", default=".")

    args = parser.parse_args()

    # Convert directory to Path object
    directory = Path(args.directory)

    # Check if directory exists
    if not directory.exists():
        print(f"Error: Directory '{directory}' does not exist.", file=sys.stderr)
        exit(1)

    if not directory.is_dir():
        print(f"Error: '{directory}' is not a directory.", file=sys.stderr)
        exit(1)

    # Initialize Magika instance
    magika = Magika()

    # Scan directory
    file_type_sizes = scan_directory(directory, magika)

    # Display results
    display_results(file_type_sizes)


if __name__ == "__main__":
    main()
