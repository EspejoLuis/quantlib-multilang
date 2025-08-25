# QuantLib Multilang

This is a personal learning project where I rewrite key components of [`QuantLib`](https://github.com/lballabio/quantlib) in **C++**, **C#**, **Python**, and **Rust**. I'm going to use ChatGPT to explain the library, help with the code, explain structure, code correction.

Link to see the daily logs: [`Devlogs`](docs/devlogs/)

## Project Structure

```text
quantlib-multilang/
├── code-cpp/
├── code-csharp/
├── code-python/
├── code-rust/
├── docs/
│   └── devlog.md
├── README.md
├── .gitignore
```

## Goals

- Learn modern C++ by rebuilding QuantLib functionality
- Practice clean design in Pythonm, C#, Rust and C++
- Compare idiomatic patterns across three languages
- Understand financial instruments and pricing engines deeply

## Getting Started

Each subfolder contains its own instructions:

- [`code-cpp/`](code-cpp/) – build with CMake
- [`code-csharp/`](code-csharp/) – .NET 9 class library
- [`code-python/`](code-python/) – uses `uv` and `ruff` and `polars`
- [`code-rust/`](code-rust/) – Rust library managed by Cargo

## License

MIT or TBD (personal use for now).
