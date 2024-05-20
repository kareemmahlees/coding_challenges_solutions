# Comprust

A compression tool written in Rust based on the Huffman encoding algorithm.

challenge url: https://codingchallenges.fyi/challenges/challenge-huffman

> [!Note]
> This challenge is only tested against text file, you might find that it works well with other file formats, but no guarantee.

## Getting started

```sh
git clone https://github.com/kareemmahlees/coding_challenges_solutions --depth=1

cd coding_challenges_solutions/comprust

# compression
cargo run -- compress .\test.txt -o .\compressed.txt

# de-compression
cargo run -- de-compress .\compressed.txt -o .\test_files\decompressed.txt

# tests
cargo test
```
