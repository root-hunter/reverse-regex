# reverse-regex 🔄🔡

**reverse-regex** is a Rust library that generates strings based on regular expressions. Given a regex pattern, the tool produces random strings that conform to the specified format. This is particularly useful for tasks such as automated testing, data generation, and fuzzing.

The project includes a demo compiled to WebAssembly, allowing you to generate strings from regex patterns directly in the browser.

## Features ✨

- 🔧 **Generate valid strings** from any regular expression pattern.
- ⚡ **Full support** for Rust standard regex syntax.
- 🌍 **WebAssembly build** for easy browser-based usage.

## Live Demo 🚀

You can try out the live demo of the project, where you can input a regular expression and get a generated string that matches it. The demo is compiled to WebAssembly for direct use in the browser.

[🎮 Try the live demo here!](https://root-hunter.github.io/reverse-regex/)

## Installation 📦

To use **reverse-regex** in your Rust project, follow these steps:

1. Add the following line to your `Cargo.toml`:
   ```toml
   [dependencies]
   reverse-regex = "0.1.0"
