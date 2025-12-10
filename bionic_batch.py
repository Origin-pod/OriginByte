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
    # Check if text has front matter (starts with ---)
    if text.startswith('---'):
        lines = text.split('\n')
        end_front_matter = -1

        # Find the end of front matter
        for i in range(1, len(lines)):
            if lines[i].strip() == '---':
                end_front_matter = i
                break

        if end_front_matter > 0:
            # Keep front matter unchanged, process the rest
            front_matter = '\n'.join(lines[:end_front_matter + 1])
            content = '\n'.join(lines[end_front_matter + 1:])
            bionic_content = transform_content(content)
            return front_matter + '\n' + bionic_content

    # No front matter, process entire text
    return transform_content(text)


def transform_content(content: str) -> str:
    """Transform content text to bionic reading style"""
    tokens = re.split(r'(\s+)', content)  # keep whitespace
    return ''.join(t if t.isspace() else bionic_word(t) for t in tokens)


# --- Process all .md files in a directory ---
def process_directory(path: str, suffix: str = "-bionic"):
    """Process all .md files, creating Bionic-Reading versions with a suffix"""
    processed_count = 0

    for root, _, files in os.walk(path):
        for file in files:
            if file.endswith(".md"):
                full_path = os.path.join(root, file)

                # Skip Bionic files to avoid processing them again
                if file.endswith(f"{suffix}.md"):
                    continue

                print(f"🔧 Processing {full_path}")

                with open(full_path, "r", encoding="utf-8") as f:
                    original = f.read()

                transformed = bionic_text(original)

                # Create Bionic version with suffix
                path_parts = os.path.splitext(full_path)
                bionic_path = f"{path_parts[0]}{suffix}{path_parts[1]}"

                with open(bionic_path, "w", encoding="utf-8") as f:
                    f.write(transformed)

                print(f"  ✨ Created {bionic_path}")
                processed_count += 1

    print(f"\n✨ Done! Created {processed_count} Bionic-Reading files with '{suffix}' suffix.")
    print(f"📖 Original files remain unchanged. Use '{suffix}' files for enhanced reading in VS Code.")


def restore_originals(path: str):
    """Restore original files from .bak backups (if they exist)"""
    restored_count = 0

    for root, _, files in os.walk(path):
        for file in files:
            if file.endswith(".md.bak"):
                bak_path = os.path.join(root, file)
                original_path = bak_path[:-4]  # Remove .bak

                print(f"🔄 Restoring {original_path}")

                with open(bak_path, "r", encoding="utf-8") as f:
                    original_content = f.read()

                with open(original_path, "w", encoding="utf-8") as f:
                    f.write(original_content)

                os.remove(bak_path)
                restored_count += 1

    if restored_count > 0:
        print(f"\n✅ Restored {restored_count} original files from .bak backups")
    else:
        print("\n❌ No .bak backup files found to restore")


def clean_bionic_files(path: str, suffix: str = "-bionic"):
    """Remove all Bionic-Reading files with the given suffix"""
    removed_count = 0

    for root, _, files in os.walk(path):
        for file in files:
            if file.endswith(f"{suffix}.md"):
                bionic_path = os.path.join(root, file)

                print(f"🗑️  Removing {bionic_path}")
                os.remove(bionic_path)
                removed_count += 1

    if removed_count > 0:
        print(f"\n✅ Removed {removed_count} Bionic-Reading files")
    else:
        print(f"\n❌ No files with '{suffix}.md' suffix found")


# --- CLI ---
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Convert Markdown files to Bionic Reading style.")
    parser.add_argument("path", help="Folder containing .md files (processed recursively)")
    parser.add_argument("--suffix", default="-bionic",
                        help="Suffix for bionic files (default: '-bionic')")
    parser.add_argument("--restore", action="store_true",
                        help="Restore original files from .bak backups")
    parser.add_argument("--clean", action="store_true",
                        help="Remove all bionic files with the suffix")

    args = parser.parse_args()

    if args.restore:
        restore_originals(args.path)
    elif args.clean:
        clean_bionic_files(args.path, args.suffix)
    else:
        process_directory(args.path, args.suffix)
