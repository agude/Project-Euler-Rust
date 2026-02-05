#!/bin/sh
# Install git hooks for this repository

HOOKS_DIR="$(git rev-parse --show-toplevel)/.hooks"
GIT_HOOKS_DIR="$(git rev-parse --git-dir)/hooks"

for hook in "$HOOKS_DIR"/*; do
    if [ -f "$hook" ]; then
        name=$(basename "$hook")
        ln -sf "$hook" "$GIT_HOOKS_DIR/$name"
        echo "Installed $name hook"
    fi
done

echo "Done."
