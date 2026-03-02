# deepwiki-cli

Query GitHub repository wikis via [DeepWiki](https://deepwiki.com) — without MCP overhead.

## Install

Homebrew (macOS/Linux)

```bash
brew tap hamsurang/deepwiki-cli
brew install deepwiki-cli
```

Cargo

```bash
cargo install deepwiki-cli
```

## Local Development

Build and run the CLI locally from source:

```bash
git clone https://github.com/hamsurang/deepwiki-cli.git
cd deepwiki-cli
```

Verify your Rust toolchain:

```bash
rustc --version
cargo --version
```

The Rust channel is pinned via `rust-toolchain.toml`.

Convenient cargo aliases are defined in `.cargo/config.toml`:

```bash
cargo ck
cargo fmt-check
cargo lint
```

Build the project:

```bash
cargo build
cargo build --release
```

Run locally with Cargo:

```bash
cargo run -- ask facebook/react "How does useEffect work?"
cargo run -- structure facebook/react
cargo run -- read facebook/react
```

Run tests:

```bash
cargo test
```

Local lint/format checks:

```bash
cargo fmt-check
cargo lint
cargo test
```

Apply formatting locally:

```bash
cargo fmt --all
```

Install typo and naming lint tools (once):

```bash
cargo install typos-cli
npm install --global @ls-lint/ls-lint
```

Run typo and naming checks:

```bash
typos
ls-lint -config .ls-lint.json
```

Troubleshooting:

- This CLI calls DeepWiki over network (`https://mcp.deepwiki.com/mcp`), so internet access is required.
- If `cargo` is not found, ensure `$HOME/.cargo/bin` is in your `PATH`.

## Usage

```bash
deepwiki-cli ask facebook/react "How does useEffect work?"
deepwiki-cli structure facebook/react
deepwiki-cli read facebook/react
```

## Token Savings

Use with [hamkit's deepwiki plugin](https://github.com/hamsurang/hamkit) instead of MCP connection:

| Method                   | Overhead                              |
| ------------------------ | ------------------------------------- |
| MCP server connection    | initialize + tool listing per session |
| deepwiki CLI (this tool) | result text only                      |

The CLI calls DeepWiki's HTTP API directly via `Bash` tool — only the result text enters Claude's context.
