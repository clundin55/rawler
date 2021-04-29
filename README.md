# Rawler

A toy web crawler written in Rust.

# Getting Started

To get start with Rawler, first install Rust, and then run `cargo run --release -- --help`. This will give you the command options available for Rawler. 

**NOTE**: You must provide the first URL for Rawler to crawl, or it will be abort the program.

Rawler will crawl through a web page, and find all hyper links in it. It will then crawl through those nested links, until a depth limit is hit.
