use std::io;

// Simple LCG step to get the next pseudo-random 32-bit integer
fn next_lcg(state: u32) -> u32 {
    state.wrapping_mul(1664525).wrapping_add(1013904223)
}

pub fn read_encrypted_file(encrypted_bytes: Vec<u8>, seed: u32) -> io::Result<String> {
    let mut current_seed = seed;
    let decrypted_bytes: Vec<u8> = encrypted_bytes
        .into_iter()
        .map(|byte| {
            current_seed = next_lcg(current_seed);
            // Extract a byte from the LCG state to XOR
            let key = (current_seed & 0xFF) as u8;
            byte ^ key
        })
        .collect();

    String::from_utf8(decrypted_bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

