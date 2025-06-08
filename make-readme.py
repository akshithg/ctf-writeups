#!/usr/bin/env python3
"""
Generates the readme.md file from CTF writeups.
"""

from pathlib import Path
from typing import List


def find_readme_files(search_path: Path = Path('.')) -> List[Path]:
    """Find all readme.md files in the directory tree."""
    return list(search_path.rglob('readme.md'))


def format_readme_entry(readme_path: Path) -> str:
    """Format a readme path into a markdown link."""
    # ./ctf/subdir/readme.md -> - [ctf/subdir](./ctf/subdir/readme.md)
    name = readme_path.parent
    return f"- [{name}]({readme_path})"

def generate_readme(output_path: Path = Path('readme.md')) -> None:
    """Generate the README.md file from found readmes."""
    readme_files = find_readme_files()

    # Sort files for consistent output
    readme_files.sort()

    with output_path.open('w', encoding='utf-8') as f:
        f.write('# CTF readmes\n\n')
        f.write('DO NOT EDIT THIS MANUALLY, run: `make readme`\n\n')

        if not readme_files:
            f.write('No readmes found.\n')
            return

        for readme_file in readme_files:
            f.write(format_readme_entry(readme_file) + '\n')

    print(f"Generated {output_path} with {len(readme_files)} readmes")


if __name__ == '__main__':
    generate_readme()
