# Wordrooter - Crack the HS.fi Sanajuuri game with bits instead of brains

WordRooter is a CLI tool designed to help solve HS.fi's daily word pyramid puzzle (Sanajuuri) from top to bottom. It uses existing letter positions and integrates new letters at each level to suggest valid Finnish words.

Currently supports only Finnish.

### Usage: 
You need to have Cargo installed.
Tool can be directly installed from crates.io with command


```
cargo install wordrooter

#example

wordrooter --start-word apu --available-letters akkullup
```

### Features
- Supports daily Sanajuuri puzzle logic.
- Suggests word paths by reusing and adding letters.
- Fast CLI tool written in Rust.

### Resources

This project utilizes word lists from the following resource:

* [Every Finnish Word by hugovk](https://github.com/hugovk/everyfinnishword)