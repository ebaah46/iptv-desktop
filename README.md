# iptv-desktop

A desktop application for streaming IPTV channels, built in Rust with a Qt 6 / QML
front end. Catalog data is fetched from the [iptv-org](https://iptv-org.github.io/api/)
API and cached locally in a redb database.

## Prerequisites

- **Rust 1.85+** — the crate uses the 2024 edition.
- **Qt 6** development packages, including the modules:
    - `QtQuick`, `QtQml`
    - `QtQuick.Controls`, `QtQuick.Layouts`, `QtQuick.Window`
    - `QtMultimedia`
- A Qt multimedia backend for playback:
    - Linux: GStreamer
    - macOS: AVFoundation (bundled)
    - Windows: Media Foundation (bundled)
- A C++ toolchain (`cc`/`clang`/`g++`), required by `cxx-qt`/`qtbridge` to compile the Qt bridge.
- `git`, to fetch the `iptv-core` dependencies.
- Network access to `https://iptv-org.github.io/api/` at runtime.

Make sure Qt is discoverable by the build, e.g. by having `qmake` on `PATH` or by
setting `QTDIR`/`QT_ROOT` to your Qt 6 installation.

## Dependencies

Declared in `Cargo.toml`:

| Crate | Purpose |
| --- | --- |
| `qtbridge` | Rust ↔ Qt/QML bridge (`cxx-qt` based) |
| `directories` | Platform-specific data/config directories |
| `serde`, `serde_json` | Configuration and API (de)serialization |
| `tokio` | Async runtime for networking and playback |
| `anyhow` | Error handling |
| `log`, `env_logger` | Logging |
| `libcore` (`core`) | Domain model and services from [iptv-core](https://github.com/ebaah46/iptv-core) |
| `libcore_api` (`iptv-adapter-api`) | REST client adapter |
| `libcore_cache` (`iptv-adapter-cache`) | Caching adapter |

The three `libcore*` crates are pulled directly from GitHub, so an internet
connection is needed on the first build.

## Building

```sh
# Debug build
cargo build

# Release build (keeps debug symbols for profiling — see Cargo.toml)
cargo build --release
```

## Running

The application loads QML by relative path (`src/qml/Main.qml`), so run it from the
project root:

```sh
cargo run
```

Log level defaults to `INFO`; override it with `RUST_LOG`:

```sh
RUST_LOG=debug cargo run
```

### Configuration

By default the embedded `config.default.json` is used. To point at a custom
configuration file, set `IPTV_CONFIG_PATH`:

```sh
IPTV_CONFIG_PATH=/path/to/config.json cargo run
```

The cache database (`cache.redb`) is written to the platform data directory
derived from the `app` identity in the configuration.

On startup the catalog is loaded from the cache while it is younger than
`cache.ttl_days`. Once it expires, the catalog is fetched from the API and the
cache is rewritten. Set `ttl_days` to `0` to always fetch from the network.