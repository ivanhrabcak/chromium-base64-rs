//! A pure rust port of chromiums's `btoa` and `atob`.
//!
//! # Example usage:
//! ```
//! let base64_encoded_string = base64_encode_to_string("Hello World!".as_bytes());
//
//! let decoded_string = base64_decode_to_string(base64_encoded_string.as_bytes());
//! ```
pub mod encode;
pub mod decode;







#[cfg(test)]
mod tests {
    use crate::{encode::base64_encode_to_string, decode::base64_decode_to_string};

    #[test]
    fn encoding_test() {
        let result = base64_encode_to_string(b"Hello World!");
        assert_eq!(result, "SGVsbG8gV29ybGQh");

        let result = base64_encode_to_string("1234567890987654321{}/[;\',.".as_bytes());
        assert_eq!(result, "MTIzNDU2Nzg5MDk4NzY1NDMyMXt9L1s7Jywu");
    }

    #[test]
    fn decoding_test() {
        let result = base64_decode_to_string(b"SGVsbG8gV29ybGQh");
        assert_eq!(result, "Hello World!");

        let result = base64_decode_to_string(b"MTIzNDU2Nzg5MDk4NzY1NDMyMXt9L1s7Jywu");
        assert_eq!(result, "1234567890987654321{}/[;\',.")
    }
}
