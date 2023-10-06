/// Encode base64 data into [`Vec<u8>`]
pub fn base64_encode(data: &[u8]) -> Vec<u8> {
    let length = data.len();

    // Lookup table for btoa(), which converts a six-bit number into the
    // corresponding ASCII character.
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let btoa_lookup = |index: i16| {
        return chars[index as usize]
    };

    let mut output: Vec<u8> = Vec::with_capacity(((4 * length / 3) + 3) & !3);
    for i in (0..length).step_by(3) {
        let mut groups_of_six: [i16; 4] = [-1; 4];
        groups_of_six[0] = (data[i] >> 2) as i16;
        groups_of_six[1] = ((data[i] & 0x03) << 4) as i16;

        if length > i + 1 {
            groups_of_six[1] |= (data[i + 1] >> 4) as i16;
            groups_of_six[2] = ((data[i + 1] & 0x0f) << 2) as i16;
        }

        if length > i + 2 {
            groups_of_six[2] |= (data[i + 2] >> 6) as i16;
            groups_of_six[3] = (data[i + 2] & 0x3f) as i16;
        }

        for k in groups_of_six {
            if k != -1 {
                output.push(btoa_lookup(k));
            } else {
                // '='
                output.push(61);
            }
        }
    }

    output
}

/// uses [`base64_encode`] and converts the result to [`String`]
pub fn base64_encode_to_string(data: &[u8]) -> String {
    String::from_utf8_lossy(&base64_encode(data)).to_string()
}