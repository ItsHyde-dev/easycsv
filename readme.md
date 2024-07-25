# EasyCsv

**easycsv** is an extremely fast command line CSV toolkit built using Rust. It provides a variety of powerful features to manipulate and analyze CSV files efficiently. With easycsv, you can filter rows, find data in specific columns using a custom query language, perform aggregations, and display results in multiple formats such as CSV and JSON. Additionally, easycsv can be seamlessly integrated with other Linux commands by using shell pipes.

## Features

- **Extremely Fast**: Optimized for speed and efficiency.
- **Filter Rows**: Quickly filter rows from a CSV file based on specified criteria.
- **Custom Query Language**: Use a specialized language to find data in particular columns with support for `=`, `!=`, and `~` (contains) operators.
- **Aggregation**: Perform aggregations such as counting the number of rows and identifying duplicate entries in columns.
- **Multiple Output Formats**: Display results in CSV or JSON format.
- **Command Line Integration**: Easily integrate with other Linux commands by piping data.

## Installation

### Using Cargo

If you have Cargo installed on your machine, you can install easycsv directly from crates.io:

```sh
cargo install easycsv
```

### Building from Source

To build easycsv from source, follow these steps:

1. **Clone the repository**:

    ```sh
    git clone https://github.com/ItsHyde-dev/easycsv.git
    ```

2. **Navigate to the project directory**:

    ```sh
    cd easycsv
    ```

3. **Build the project**:

    ```sh
    cargo install --path .
    ```

4. The compiled binary will be located in the `target/release` directory. You can add this directory to your PATH or move the binary to a directory that is already in your PATH.

## Usage

### Basic Usage

```sh
easycsv [OPTIONS] <CSV_FILE>
```

### Examples

- **Find data in specific columns**:

  ```sh
  easycsv --find "name ~ 'John' and age != 25" data.csv
  ```

- **Count rows**:

  ```sh
  easycsv --count data.csv
  ```

- **Count duplicate entries in columns**:

  ```sh
  easycsv --dc "email" data.csv
  ```

- **Display results in JSON format**:

  ```sh
  easycsv --display-json data.csv
  ```

- **Integration with other commands**:

  ```sh
  cat data.csv | easycsv -f "name ~ 'Doe' or age = 30"
  ```

### Options

- `--find <CONDITION>`: Filter rows based on the specified condition.
- `--count`: Count the number of rows in the CSV file.
- `--dc <COLUMN>`: Count duplicate entries in the specified column.

You can use easycsv --help for a detailed list of available options.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.txt) file for details.

---

**easycsv**: The ultimate command line CSV toolkit for efficient data manipulation and analysis.

---
