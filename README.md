# Scrapedia

Simple CLI App for Scraping Products from Tokopedia, Written in Rust.

## Installation

Before you can use Scrapedia, you need to have Rust installed on your system. Follow these steps to install Rust:

1. Visit [rustup.rs](https://rustup.rs/) in your web browser.
2. Follow the instructions on the website to download and install Rust using `rustup`, the official Rust toolchain installer.

## Usage

Once you have Rust installed, you can use Cargo, Rust's package manager and build system, to install and run My CLI Rust App.

To run scrapedia, run the following command in your terminal:

1. Clone the repository:

```bash
git clone https://github.com/furqoncreative/scrapedia
```
2. Change directory to the project folder:
```bash
cd scrapedia
```
3. Build and install the CLI app using Cargo:
```bash
cargo run -- [OPTIONS]

Options:
  -q, --query        Product keyword [default: Samsung]
  -h, --help         Display this help message
  -v, --version      Display version information
```

## Example
```bash
cargo run -- -q iphone
```
