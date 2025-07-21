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

## 21 July 2025 - Day Two: setting python environment

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
    