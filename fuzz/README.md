This directory contains the fuzz tests for serde-bench. To fuzz, we use the `cargo-fuzz` package.

## Installation

You may need to install the `cargo-fuzz` package to get the `cargo fuzz` subcommand. Use

```sh
$ cargo install cargo-fuzz
```

`cargo-fuzz` is documented in the [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html).

## Running the fuzzer

Once `cargo-fuzz` is installed, you can run the `bincode` fuzzer with
```sh
cargo fuzz run bincode
```

You should see output that looks something like this:

```
INFO: Seed: 2073236486
INFO: Loaded 1 modules   (16636 inline 8-bit counters): 16636 [0x55cf2eddac48, 0x55cf2edded44), 
INFO: Loaded 1 PC tables (16636 PCs): 16636 [0x55cf2edded48,0x55cf2ee1fd08), 
INFO:       26 files found in /home/nicholas/bench/fuzz/corpus/bincode
INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 4096 bytes
INFO: seed corpus: files: 26 min: 1b max: 86b total: 1135b rss: 127Mb
#27     INITED cov: 2901 ft: 3189 corp: 21/833b exec/s: 0 rss: 172Mb
#2048   pulse  cov: 2901 ft: 3189 corp: 21/833b lim: 100 exec/s: 1024 rss: 174Mb
```
It will continue to generate random inputs forever, until it finds a bug or is terminated. The testcases for bugs it finds go into `fuzz/artifacts/bincode` and you can rerun the fuzzer on a single input by passing it on the command line `cargo fuzz run bincode my_testcase`.
