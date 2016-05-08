# Rust-Whitespace
A whitespace interpreter written in Rust

## Usage
<pre>
Usage:    whitespace [command] &lt;file&gt;
Commands: run   - (default) run the program
          list  - list the commands that the file contains
</pre>

## Design
The program is a simple two-pass compiler that returns a vector of operations for a virtual machine to execute.

## Build
There are no dependencies, so you should be able to run

```
git clone https://github.com/Jayshua/rust-whitespace
cd rust-whitespace
cargo run
```