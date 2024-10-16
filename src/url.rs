pub fn encode(bytes: &[u8]) -> String {
    let mut encoded_bytes = String::with_capacity(3 * bytes.len());
    for &byte in bytes {
        encoded_bytes.push('%');
        encoded_bytes.push_str(&hex::encode([byte]));
    }

    encoded_bytes
}
