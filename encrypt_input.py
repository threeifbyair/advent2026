import struct
import sys

def next_lcg(state: int) -> int:
    # Mimic 32-bit unsigned wrapping arithmetic: (state * 1664525 + 1013904223) & 0xFFFFFFFF
    return (state * 1664525 + 1013904223) & 0xFFFFFFFF

def encrypt_file(input_path: str, output_path: str, seed: int):
    try:
        with open(input_path, 'r', encoding='utf-8') as f:
            data = f.read()
    except FileNotFoundError:
        print(f"Error: {input_path} not found.")
        sys.exit(1)

    input_bytes = data.encode('utf-8')
    output_bytes = bytearray()
    
    current_seed = seed
    for byte in input_bytes:
        current_seed = next_lcg(current_seed)
        key = current_seed & 0xFF
        output_bytes.append(byte ^ key)
        
    with open(output_path, 'wb') as f:
        f.write(output_bytes)
    print(f"Successfully encrypted {input_path} -> {output_path} (Seed: {seed})")

if __name__ == "__main__":
    # Example usage matching the previous seed
    encrypt_file("input.txt", "encrypted.bin", seed=42)
