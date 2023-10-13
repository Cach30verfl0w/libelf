<div align = "center">

# `LibELF`
![GitHub](https://img.shields.io/github/license/Cach30verfl0w/libelf) ![GitHub issues](https://img.shields.io/github/issues/Cach30verfl0w/libelf) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Cach30verfl0w/libelf) ![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/y/Cach30verfl0w/libelf) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Cach30verfl0w/libelf/main)
![GitHub pull requests](https://img.shields.io/github/issues-pr/Cach30verfl0w/libelf)

LibELF is a library for parsing and loading ELF files from memory or files as an project for learning purposes

</div>

## Safety
I aim to use as few unsafe blocks as possible, but I had to use a few ([Elf::from_bytes](https://github.com/Cach30verfl0w/libelf/blob/main/src/lib.rs#L70-L74)). This library is as safe as possible for me. I can remove a lot unsafe blocks, when Rust contains [safe transmutes](https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html).

## Related projects
I found some projects that are related to this. A few of them are written in a different language, but you can check out them too. Here is a list with them. Many of them are also better and more production-ready than this crate.
- [rust-elf](https://github.com/cole14/rust-elf/) by [cole14](https://github.com/cole14)
