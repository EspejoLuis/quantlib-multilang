# QuantLib Multilang

This is a personal learning project where I rewrite key components of the QuantLib C++ library in **C++**, **C#**, and **Python**. I'm going to use ChatGPT to explain the library, help with the code and explain.

## Project Structure

```text
quantlib-multilang/
├── code-cpp/
├── code-csharp/
├── code-python/
├── docs/
│   └── devlog.md
├── README.md
├── .gitignore
```

## Goals

- Learn modern C++ by rebuilding QuantLib functionality
- Practice clean design in Python and C#
- Compare idiomatic patterns across three languages
- Understand financial instruments and pricing engines deeply

## Getting Started

Each subfolder contains its own instructions:
- [`code-cpp/`](code-cpp/) – build with CMake
- [`code-csharp/`](code-csharp/) – .NET 9 class library
- [`code-python/`](code-python/) – uses `uv` and `ruff` and `polars`

## License

MIT or TBD (personal use for now).
