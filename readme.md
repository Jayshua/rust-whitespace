# Rust-Whitespace
A whitespace interpreter written in Rust

## Usage
<pre>
Usage:    whitespace [command] &lt;file&gt;
Commands: run   - (default) run the program
          list  - list the commands that the file contains
</pre>

## Design
It's built as a simple two-pass compiler that returns a vector of operations for a virtual machine to execute.

The first pass converts the whitespace tokens into a processable enum, and the second pass reduces the named labels to simple program-pointers. (Like line numbers, but indexes.)

## Build
There are no dependencies, so you should be able to run

```
git clone https://github.com/Jayshua/rust-whitespace
cd rust-whitespace
cargo run
```