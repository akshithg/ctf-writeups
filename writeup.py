#!/usr/bin/env python3
"""
Generates the readme.md file from CTF writeups.
"""

from pathlib import Path
from typing import List


def find_writeup_files(search_path: Path = Path('.')) -> List[Path]:
    """Find all writeup.md files in the directory tree."""
    return list(search_path.rglob('writeup.md'))


def format_writeup_entry(writeup_path: Path) -> str:
    """Format a writeup path into a markdown link."""
    # ./ctf/subdir/writeup.md -> - [ctf/subdir](./ctf/subdir/writeup.md)
    name = writeup_path.parent
    return f"- [{name}]({writeup_path})"

def generate_readme(output_path: Path = Path('readme.md')) -> None:
    """Generate the README.md file from found writeups."""
    writeup_files = find_writeup_files()

    # Sort files for consistent output
    writeup_files.sort()

    with output_path.open('w', encoding='utf-8') as f:
        f.write('# CTF writeups\n\n')
        f.write('DO NOT EDIT THIS MANUALLY, run: `make readme`\n\n')

        if not writeup_files:
            f.write('No writeups found.\n')
            return

        for writeup_file in writeup_files:
            f.write(format_writeup_entry(writeup_file) + '\n')

    print(f"Generated {output_path} with {len(writeup_files)} writeups")


if __name__ == '__main__':
    generate_readme()
