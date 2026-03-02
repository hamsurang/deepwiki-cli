#!/bin/sh

# PostToolUse payload is delivered through stdin.
payload=$(cat)

path=$(printf '%s' "$payload" | sed -n 's/.*"file_path"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p')
if [ -z "$path" ]; then
  path=$(printf '%s' "$payload" | sed -n 's/.*"path"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p')
fi

case "$path" in
  *.rs)
    cargo fmt --all >/dev/null 2>&1 || true
    ;;
  *)
    ;;
esac

exit 0
