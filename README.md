# Overview

Learning project to attempt to build a JSON parser/stringify module in Rust.
Comprised of:
- lexer module, to generate tokens from raw string
- parser module, to build Vec & HashMap structures from sequences of tokens
- json module, to expose public methods (parse/stringify)

## Run

```bash
cargo watch -c -w src -x run
```

## Clean

```bash
cargo clean
```