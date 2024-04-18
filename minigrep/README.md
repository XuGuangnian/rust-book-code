## minigrep

## Usage

```shell
cargo run <query> <filename>
```

- query: the string you want to search for
- filename: the name of the file you want to search in

### Examples

1. console(default): search results will be printed to the console
    ```shell
    cargo run to poem.txt
    ```
2. env: IGNORE_CASE=1: search results will be case-insensitive
    ```shell
    IGNORE_CASE=1 cargo run to poem.txt
    ```
   or in PowerShell
    ```powershell
    $env:IGNORE_CASE=1; cargo run to poem.txt
    ```
3. file: `> <resultFile>`: search results will be written to the file
    ```shell
    cargo run to poem.txt > output.txt
    ```

