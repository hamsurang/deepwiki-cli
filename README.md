# deepwiki-cli

Query GitHub repository wikis via [DeepWiki](https://deepwiki.com) — without MCP overhead.

## Install

**Homebrew (macOS/Linux)**

```bash
brew tap hamsurang/deepwiki-cli
brew install deepwiki-cli
```

**Cargo**

```bash
cargo install deepwiki-cli
```

## Usage

```bash
deepwiki ask facebook/react "How does useEffect work?"
deepwiki structure facebook/react
deepwiki read facebook/react
```

## Token Savings

Use with [hamkit's deepwiki plugin](https://github.com/hamsurang/hamkit) instead of MCP connection:

| Method | Overhead |
|--------|----------|
| MCP server connection | initialize + tool listing per session |
| deepwiki CLI (this tool) | result text only |

The CLI calls DeepWiki's HTTP API directly via `Bash` tool — only the result text enters Claude's context.
