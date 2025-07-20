# QuantLib Multilang

This is a personal learning project where I rewrite key components of the QuantLib C++ library in **C++**, **C#**, and **Python**.

## Project Structure

quantlib-multilang/
├── code-cpp/           # My C++ rewrite of QuantLib
├── code-csharp/        # C# port of QuantLib logic
├── code-python/        # Python implementation
├── docs/          # Internal documentation folder
│   └── devlog.md  # My daily notes and learning log
├── README.md      # Project overview (public-facing)
├── .gitignore     # Files/folders to exclude from version control


## Goals

- Learn modern C++ by rebuilding QuantLib functionality
- Practice clean design in Python and C#
- Compare idiomatic patterns across three languages
- Understand financial instruments and pricing engines deeply

## Getting Started

Each subfolder contains its own instructions:
- [`cpp/`](cpp/) – build with CMake
- [`csharp/`](csharp/) – .NET 8 class library
- [`python/`](python/) – uses `uv` and `ruff`

## License

MIT or TBD (personal use for now).
