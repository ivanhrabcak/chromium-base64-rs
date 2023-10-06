# chromium-base64-rs
Pure rust implementation of chromium's btoa and atob

# Dependency
```toml
chromium-base64-rs = "0.1.0"
```

# Example
Here's a encoding and decoding example:
```rust
let base64_encoded_string = base64_encode_to_string("Hello World!".as_bytes());

let decoded_string = base64_decode_to_string(base64_encoded_string.as_bytes());
```
