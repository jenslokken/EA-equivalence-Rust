A Rust implementation of Nikolay Kaleyski's [EA equivalence algorithm](https://link.springer.com/article/10.1007/s12095-021-00513-y). 

## Usage
This program takes two inputs, `f` and `g`  which are two files representing the vectorial Boolean functions in truth table form. The first line is the dimension `n` and the second line contains all the entries of the truth table space separated:
```
6
0 1 8 15 27 14 35 48 53 39 43 63 47 41 1 1 41 15 15 47 52 6 34 22 20 33 36 23 8 41 8 47 36 52 35 53 35 39 20 22 33 34 48 53 39 48 6 23 22 33 63 14 23 52 14 43 27 63 36 6 27 43 20 34 
```

```sh
Usage: EA-equivalence-Rust [path_to_f][path_to_g]
```

## Installation

Make sure you have rustc version at least ` 1.69.0` installed.
Build using:

```
cargo install
cargo build --release
```

Run the binary found in
```
/target/release
```