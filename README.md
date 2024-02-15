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
### command
```bash
cargo run -- -q iphone
```
### output (products.json)
```json
[
  {
    "product_name": "IBOX Apple iPhone 15 Pro 128GB 256GB 512GB 1TB Blue Natural Titanium",
    "rating": "5.0",
    "store_name": "Studio Ponsel",
    "store_location": "Jakarta Pusat",
    "purchase_amount": "500+ terjual",
    "discount_info": "16%",
    "product_image_url": "https://images.tokopedia.net/img/cache/200-square/VqbcmM/2023/10/27/0c56f8cc-e374-4e8a-a691-88a398c7c3d9.jpg"
  },
  {
    "product_name": "iPhone 14 Garansi Resmi",
    "rating": "5.0",
    "store_name": "igoods gadget",
    "store_location": "Jakarta Barat",
    "purchase_amount": "750+ terjual",
    "discount_info": "17%",
    "product_image_url": "https://images.tokopedia.net/img/cache/200-square/VqbcmM/2023/8/25/a6326dd1-8334-4fd7-821a-5ace01e12c2e.png"
  }
]
```
