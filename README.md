# 𝚐𝚘𝚓𝚒

> An interactive CLI for running package.json scripts — fast, filterable, and package-manager-aware.

`goji` replaces the `npm run <tab><tab>` dance with a polished TUI that lets you search and select any script in your project in seconds.

---

## Features

- **Interactive script picker** — fuzzy-filterable list, keyboard-driven, scroll-friendly
- **Auto-detects your package manager** — supports npm, Yarn, pnpm, and Bun via lockfile detection
- **Project-root aware** — walks up the directory tree to find your `package.json`, so it works from any subdirectory
- **Exit-code passthrough** — the exit code of the chosen script is forwarded, making `goji` safe in CI pipelines and `&&` chains
- **Zero configuration** — no config files, no environment variables required

---

## Demo

```
┌  𝚐𝚘𝚓𝚒
│
◆  ─────────────────────────────────────────────────────────
│  🔎 |
│  › build
│    dev
│    lint
│    test
│    typecheck
└─────────────────────────────────────────────────────────────

✓ running pnpm build
```

---

## Installation

### From crates.io

```sh
cargo install goji-cli
```

### From source

```sh
git clone https://github.com/neiforfaen/goji
cd goji
cargo install --path .
```

> **Requirements:** Rust 1.85+ (edition 2024)

### From shell

```sh
curl -sSL https://raw.githubusercontent.com/neiforfaen/goji/main/install.sh | sh
```

---

## Usage

Run `goji` from anywhere inside a JavaScript/TypeScript project:

```sh
goji
```

That's it. `goji` will:

1. Walk up from your current directory until it finds a lock file (`package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`, `bun.lockb`) or a `.git` directory.
2. Detect the package manager from that lock file.
3. Load and sort the scripts defined in the root `package.json`.
4. Open an interactive, filterable prompt for you to pick a script.
5. Run the selected script with the correct package manager and forward its exit code.

### Filtering

Start typing as soon as the prompt appears — the list filters in real-time. Use the arrow keys to move between results and press **Enter** to confirm.

---

## Supported Package Managers

| Lock file           | Package manager |
| ------------------- | --------------- |
| `pnpm-lock.yaml`    | pnpm            |
| `yarn.lock`         | Yarn            |
| `bun.lockb`         | Bun             |
| `package-lock.json` | npm (default)   |

When multiple lock files are present, detection follows the priority order shown above.

---

## Development

```sh
# Run all tests (unit + integration)
cargo test

# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt
```

---

## Contributing

Issues and pull requests are welcome at [github.com/neiforfaen/goji](https://github.com/neiforfaen/goji).

Please run `cargo fmt` and `cargo test` before opening a PR — the CI pipeline enforces both.
