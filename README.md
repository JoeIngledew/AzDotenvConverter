# Az Dotenv Converter

This is a command-line tool to convert between Azure appsettings JSON format and dotenv format

## Usage

Running directly:

```sh
# Converting from appsettings to dotenv
cargo run -- az-to-env <INPUT> <OUTPUT>

# Converting from dotenv to appsettings
cargo run -- env-to-az <INPUT> <OUTPUT>

# example
cargo run -- env-to-az ./.dotenv ./output.json
```

Alternatively, you can build and run the binary

## Running tests

`cargo test`