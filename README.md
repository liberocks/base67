# Base67

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