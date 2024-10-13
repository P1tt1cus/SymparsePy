# symparsepy

`symparsepy` is a Rust-based Python package for parsing symbols from PDB files. This package leverages the `pyo3` library to create Python bindings for Rust code.

## Installation

To install `symparsepy`, you need to build the package into a wheel file and then install it using `pip`. Follow these steps:

1. **Install Maturin**:
    ```sh
    pip install maturin
    ```

2. **Build the Wheel**:
    Run the following command in your terminal to build the wheel file:
    ```sh
    maturin build
    ```

    This will generate a `.whl` file in the `target/wheels` directory.

3. **Install the Wheel**:
    You can install the generated wheel file using `pip`:
    ```sh
    pip install target/wheels/symparsepy-0.1.0-cp39-cp39-win_amd64.whl
    ```

## Usage

Here is an example of how to use `symparsepy` in your Python code:

### Parsing PDB Files

The `parse_pdb` function parses symbols from a PDB file and returns a list of dictionaries, each containing the parsed symbol information.

```python
import symparsepy

# Example usage
result = symparsepy.parse_pdb("path_to_pdb_file.pdb")
for symbol in result:
    print(symbol)
```

Each dictionary includes the following keys:

- `offset`: The offset value of the symbol.
- `section`: The section value of the symbol.
- `name`: The name of the symbol.
- `code`: Indicates if the symbol is code.
- `msil`: Indicates if the symbol is MSIL.
- `function`: Indicates if the symbol is a function.

### Example Output

```python
[
    {
        "offset": "421376",
        "section": "1",
        "name": "symbol_name",
        "code": "true",
        "msil": "false",
        "function": "true"
    },
    ...
]
```

### Searching Symbols

The `search_symbols` function searches for symbols in a PDB file that match a given regex pattern and returns a list of dictionaries with the matching symbols.

```python
import symparsepy

# Example usage
pattern = r"some_regex_pattern"
result = symparsepy.search_symbols("path_to_pdb_file.pdb", pattern)
for symbol in result:
    print(symbol)
```

Each dictionary includes the same keys as in the `parse_pdb` function.

## Development

To contribute to this project, follow these steps:

1. Clone the repository:
    ```sh
    git clone https://github.com/P1tt1cus/SymparsePy
    cd SymparsePy
    ```

2. Install the development dependencies:
    ```sh
    pip install maturin
    ```

3. Build the project:
    ```sh
    maturin build
    ```

## License

This project is licensed under the MIT License.

