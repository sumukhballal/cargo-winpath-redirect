# Cargo WinPathRedirect

`cargo-winpath-redirect` is a utility for Windows Rust developers facing issues with long file paths during the build process, particularly common with ESP-IDF projects or deeply nested project structures.

It transparently wraps your `cargo` commands, temporarily copying your project to a very short path (e.g., `C:\b\<project_name>`), executing the command there, and then (optionally) copying build artifacts back. This helps bypass the legacy MAX_PATH (260 character) limitations still present in some parts of Windows toolchains.

## The Problem

Windows has a historical path length limit that can cause build failures in Rust projects with deep directory structures or long file names, especially when combined with toolchains like ESP-IDF that generate many intermediate files. Errors like "The system cannot find the path specified. (os error 3)" or "failed to list cmake-file-api reply directory" are common symptoms.

## The Solution

`cargo-winpath-redirect` acts as a "subcommand" for Cargo. You invoke it like `cargo winpath-redirect <your_usual_cargo_command_and_args>`. It will:

1.  Identify your current project.
2.  Copy the project (excluding `target/` and `.git/`) to a configurable short base path (default: `C:\b\`).
3.  Change to the short path directory.
4.  Run `cargo clean` (recommended for a clean slate).
5.  Execute your specified `cargo` command (e.g., `build`, `run`, `test`) with all its arguments.
6.  Stream all output (stdout/stderr) back to your console and allow stdin interaction.
7.  If the command produces build artifacts (e.g., `build`, `run`), copy the `target` directory from the short path back to your original project's `target` directory.
8.  Clean up the temporary short path directory (unless configured otherwise).

## Installation

1.  Ensure you have Rust and Cargo installed.
2.  Clone this repository:
    ```bash
    git clone https://github.com/sumukhballal/cargo-winpath-redirect.git
    cd cargo-winpath-redirect
    ```
3.  Install the tool:
    ```bash
    cargo install --path .
    ```
    This will build `cargo-winpath-redirect.exe` and place it in your `~/.cargo/bin` directory, which should be in your system `PATH`.

## Usage

Once installed, navigate to your Rust project directory that experiences path length issues. Then, simply prefix your usual `cargo` commands with `winpath-redirect`:

**Instead of:**
```bash
cargo build --release --target riscv32imc-esp-espidf
cargo run -- --my-app-arg
cargo test
cargo clean

Use:

cargo winpath-redirect build --release --target riscv32imc-esp-espidf

or 

cargo winpath-redirect run -- --my-app-arg

or 

cargo winpath-redirect test

or 

cargo winpath-redirect clean
```

## Contributing

Contributions, bug reports, and feature requests are welcome! Please open an issue or pull request on the GitHub repository.
