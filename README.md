# traxer

traxer is a thin wrapper around `tracing` and `tracing-subscriber`, with CLI-friendly defaults.

## What it does

- Minimal setup for plain/json logs
- Safe init helpers (`init`, `try_init`, `is_initialized`)
- CLI-friendly defaults (`stderr`, env-aware filtering, color auto detection)
- Optional extras (`span`, `error_report`, base fields such as `pid`/`exe`/`version`)

## Quick start

```rust
fn main() {
    let cfg = traxer::Config::new("my-cli")
        .json()
        .span(true);
    traxer::init(cfg);

    traxer::info!("hello from traxer");
}
```
