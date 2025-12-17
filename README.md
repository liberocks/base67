# Base67

![Build Status](https://github.com/liberocks/base67/actions/workflows/build.yml/badge.svg)
[![Coverage](https://coveralls.io/repos/github/liberocks/base67/badge.svg?branch=master)](https://coveralls.io/github/liberocks/base67?branch=master)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/6d7316eb78084cc6bbe1152ee7ac51f7)](https://app.codacy.com/gh/liberocks/base67/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Contributors](https://img.shields.io/github/contributors/liberocks/base67)](https://github.com/liberocks/base67/graphs/contributors)
[![License](https://img.shields.io/github/license/liberocks/base67)](./LICENSE)

Base67 is a base64-like encoding scheme that uses a 67-character alphabet.

### Usage

```rust
use base67::{encode, decode};

fn main() {
    // Encode a string
    let data = b"Hello, World!";
    let encoded = encode(data);
    println!("Encoded: {}", encoded);
    
    // Decode back to original
    let decoded = decode(&encoded).unwrap();
    println!("Decoded: {}", String::from_utf8(decoded).unwrap());
}
```