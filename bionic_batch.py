#!/usr/bin/env python3
import os
import re
import argparse

# - to restore all the files from .bak
# find . -name "*.md.bak" -exec sh -c 'mv "$1" "${1%.bak}"' _ {} \; 


# --- Bionic transformation for a single word ---
def bionic_word(word: str) -> str:
    m = re.match(r'^(\W*)(\w+)(\W*)$', word)
    if not m:
        return word

    leading, core, trailing = m.groups()
    length = len(core)

    # Rule (you can tweak):
    if length <= 3:
        bold_count = 0
    elif length <= 7:
        bold_count = 2
    else:
        bold_count = 3

    if bold_count == 0:
        return word

    return f"{leading}**{core[:bold_count]}**{core[bold_count:]}{trailing}"


# --- Transform a whole markdown file's text ---
def bionic_text(text: str) -> str:
    tokens = re.split(r'(\s+)', text)  # keep whitespace
    return ''.join(t if t.isspace() else bionic_word(t) for t in tokens)


# --- Process all .md files in a directory ---
def process_directory(path: str, backup: bool):
    for root, _, files in os.walk(path):
        for file in files:
            if file.endswith(".md"):
                full_path = os.path.join(root, file)

                print(f"🔧 Processing {full_path}")

                with open(full_path, "r", encoding="utf-8") as f:
                    original = f.read()

                transformed = bionic_text(original)

                # Optional backup
                if backup:
                    with open(full_path + ".bak", "w", encoding="utf-8") as b:
                        b.write(original)

                # Replace in place
                with open(full_path, "w", encoding="utf-8") as f:
                    f.write(transformed)

    print("\n✨ Done! All .md files have been updated.")


# --- CLI ---
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Convert Markdown files to Bionic Reading style.")
    parser.add_argument("path", help="Folder containing .md files (processed recursively)")
    parser.add_argument("--backup", action="store_true",
                        help="Create .bak copies before overwriting")
    args = parser.parse_args()

    process_directory(args.path, args.backup)
