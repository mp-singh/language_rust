# Language Rust - Simple gRPC Learning Project

This is an example Rust project created for learning purposes. It uses the Tonic library for gRPC, the Tokio library for asynchronous programming, and the SQLx library for database operations. It also includes a logging setup using the simplelog library.

The project demonstrates how to set up a gRPC server and client, perform database operations, and handle asynchronous tasks in Rust.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust: You can download Rust from [the official website](https://www.rust-lang.org/tools/install).

### Installing

1. Clone the repository
    ```bash
    git clone https://github.com/mp-singh/language_rust.git
    ```
2. Navigate to the project directory
    ```bash
    cd language_rust
    ```
3. Build the project
    ```bash
    cargo build
    ```
4. Run the project
    ```bash
    cargo run
    ```

## Running the tests

You can run the tests with the following command:

```bash
cargo test
```

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used
- [Tonic](https://github.com/hyperium/tonic) - The gRPC library used
- [Tokio](https://tokio.rs/) - The asynchronous runtime used
- [SQLx](https://github.com/launchbadge/sqlx) - The database library used
- [simplelog](https://crates.io/crates/simplelog) - The logging library used

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
