/// Decode base64 data into [`Vec<u8>`]
pub fn base64_decode(data: &[u8]) -> Vec<u8> {
    let length = data.len();

    let mut data = data.to_vec();
    if length % 4 == 0 {
        // '='
        if data[length - 1] == 61 && data[length - 2] == 61 {
            data.truncate(length - 2);
        } else if data[length - 1] == 61 {
            data.truncate(length - 1);
        }
    }

    let allowed_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let atob_lookup = |ch: i32| {
        for (i, k) in allowed_chars.iter().enumerate() {
            if ch == *k as i32 {
                return i as i32
            }
        }

        unreachable!()
    };

    let mut output: Vec<u8> = Vec::new();

    let mut buffer: i32 = 0;
    let mut accumulated_bits = 0;

    for ch in data {
        buffer <<= 6;
        buffer |= atob_lookup(ch.into());

        accumulated_bits += 6;

        if accumulated_bits == 24 {
            output.push(((buffer & 0xff0000) >> 16) as u8);
            output.push(((buffer & 0xff00) >> 8) as u8);
            output.push((buffer & 0xff) as u8);

            buffer = 0;
            accumulated_bits = 0;
        }
    }

    if accumulated_bits == 12 {
        buffer >>= 4;
        output.push(buffer as u8);
    } else if accumulated_bits == 18 {
        buffer >>= 2;
        output.push(((buffer & 0xff00) >> 8) as u8);
        output.push((buffer & 0xff) as u8);
    }

    return output;
}

/// uses [`base64_decode`] and converts the result to [`String`]
pub fn base64_decode_to_string(data: &[u8]) -> String {
    return String::from_utf8_lossy(base64_decode(data).as_slice()).to_string()
}