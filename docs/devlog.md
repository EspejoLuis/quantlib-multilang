# July

## 20 July 2025 - Day One: setting environements and structure folder

- Setting Environments:
    - Quantlib 1-38 already installed in the past so we are good. It was a lengthy and somehow complex process but got it working.

-  Using ChatGpt to help me with suggestiongs about structure of folders. We can have one single repo and three folders. One for each language. Then later if needed we can extract folders:

    - Python folder --> [`code-python`](code-python)
    - C# folder--> [`code-csharp`](code-csharp)
    - c++ folder --> [`code-cpp`](code-ccp)

- Had to do some fixing: 
    - git remote remove origin
    - then just using source control bar --> publish Branch

## 21 July 2025 - Day Two: setting python and c# environment

- Python:
    - `uv venv --python=/opt/homebrew/bin/python3.11` to create the uv environemnt with python 3.11
    - activate environment `source .venv/bin/activate`
    - `uv pip install -U pip setuptools wheel` to install some basic libraries
    - create the pyproject.toml with polars version.
    - `uv pip install -r pyproject` to install the content in pyproject.
    - added ruff extension to visual studio. Check in extensions.
    - create ruff.toml setting file.
    - create setting.json in .vscode for using ruff.
    - selet the python interpreter : cmd+shift+P --> .venv should appear. If not the add it manually using the address found in `which python`
    
- C#:
    - `dotnet new classlib --name QuantLibCSharp --output .` to set up the C# environment
    - build it `dotnet build`

## 22 July 2025 - Day Three: setting C++ environment

- C++:
    - in code-cpp folder [`cd code-cpp`]:
        - `mkdir src`
        - `touch src/main.cpp`
        - `touch CMakeLists.txt`
    - add specs in CMakeLists.txt and example in main.ccp to test
    - then:
        - `mkdir build`
        - `cd build`
        - `cmake ..` search for a CMakeLists file in the parent folder `code-ccp`, find the source files to compile `src/main.cpp`, set up Makefile in `build` folder.
        - `make` :
            - compile `main.ccp` into `main.ccp.o`(object file in `CMakeFiles/quantlib_ccp.dir/src/`), `main.ccp.o.d` is created as well.
                - `main.cpp.o` -> compiled binary blob
                - `main.cpp.o.d` -> dependency file for header tracking
            - link all objects into final binary `quantlib_ccp`(in `build/`).
        - `./quantlib_cpp` execute! (from `build/`)