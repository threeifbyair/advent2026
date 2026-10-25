#!/usr/bin/env python3
import struct
import sys
import argparse
import os

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
    parser = argparse.ArgumentParser(description="Encrypt an input file with a given seed.")
    parser.add_argument("input", help="Input file to encrypt")
    parser.add_argument("output", help="Output file for the encrypted data")
    parser.add_argument("--seed", type=int, default=os.environ.get("AOC_SEED", 42), help="Encryption seed (default: 42)")

    args = parser.parse_args()

    encrypt_file(args.input, args.output, seed=args.seed)
