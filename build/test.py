# Create a minimal valid NEF file for testing
import struct

def create_minimal_nef():
    nef = bytearray()
    
    # Magic: NEF3
    nef.extend(b'NEF3')
    
    # Compiler (64 bytes)
    compiler = b'neo-compiler'
    nef.extend(compiler)
    nef.extend(b'\x00' * (64 - len(compiler)))
    
    # Source (1 byte length + 255 bytes)
    source = b'test'
    nef.append(len(source))
    nef.extend(source)
    nef.extend(b'\x00' * (255 - len(source)))
    
    # Reserved (1 byte)
    nef.append(0)
    
    # Tokens (1 byte count)
    nef.append(0)  # No tokens
    
    # Reserved (2 bytes)
    nef.extend(b'\x00\x00')
    
    # Script
    script = bytes([
        0x21,  # PUSH1 (push 1 to stack)
        0x40   # RET (return)
    ])
    nef.extend(struct.pack('<I', len(script)))  # Script length (4 bytes, little-endian)
    nef.extend(script)
    
    # Calculate checksum (simple sum for now - should be proper CRC32)
    # For testing, we'll use a placeholder
    checksum = 0x12345678
    nef.extend(struct.pack('<I', checksum))
    
    return bytes(nef)

# Write the NEF file
with open('build/minimal.nef', 'wb') as f:
    f.write(create_minimal_nef())

print("Created minimal.nef")
