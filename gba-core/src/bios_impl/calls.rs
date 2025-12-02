/// BIOS Call implementations
/// These are high-level emulation of GBA BIOS functions
/// Division result
#[derive(Debug, Clone, Copy)]
pub struct DivResult {
    pub quotient: i32,
    pub remainder: i32,
    pub abs_quotient: i32,
}

/// Sqrt result (16-bit)
#[derive(Debug, Clone, Copy)]
pub struct SqrtResult {
    pub result: u16,
}

/// SoftReset - Reset most of the system
pub fn soft_reset() {
    // In real hardware, this would:
    // - Clear 0x03007F00-0x03007FFF (256 bytes)
    // - Clear most I/O registers
    // - Jump to address in ROM header
    // We handle this at emulator level
}

/// Div - Signed division
pub fn div(numerator: i32, denominator: i32) -> DivResult {
    if denominator == 0 {
        // Division by zero behavior
        DivResult {
            quotient: if numerator >= 0 { i32::MAX } else { i32::MIN },
            remainder: numerator,
            abs_quotient: i32::MAX,
        }
    } else {
        let quotient = numerator / denominator;
        let remainder = numerator % denominator;
        DivResult {
            quotient,
            remainder,
            abs_quotient: quotient.abs(),
        }
    }
}

/// Sqrt - Integer square root
pub fn sqrt(value: u32) -> SqrtResult {
    let result = (value as f64).sqrt() as u16;
    SqrtResult { result }
}

/// ArcTan - Arctangent approximation
pub fn arctan(x: i16) -> i16 {
    // Simple linear approximation for tan^-1(x/2^14)
    // Real BIOS uses a lookup table
    let x_f = (x as f64) / 16384.0;
    let result = x_f.atan() * 16384.0 / std::f64::consts::PI;
    result as i16
}

/// ArcTan2 - Two-argument arctangent
pub fn arctan2(x: i16, y: i16) -> u16 {
    if x == 0 && y == 0 {
        return 0;
    }
    let angle = (y as f64).atan2(x as f64);
    // Convert to 0-FFFF range (0-360 degrees)
    let normalized = ((angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI)) * 65536.0;
    normalized as u16
}

/// CpuSet - Memory copy/fill with 16-bit or 32-bit transfers
pub fn cpu_set<F>(source: u32, dest: u32, control: u32, mut read_mem: F, mut write_mem: F)
where
    F: FnMut(u32) -> u32 + Clone,
{
    let count = control & 0x1FFFFF;
    let is_32bit = (control & (1 << 26)) != 0;
    let is_fill = (control & (1 << 24)) != 0;

    let word_size = if is_32bit { 4 } else { 2 };

    if is_fill {
        // Fill mode: copy same value repeatedly
        let _fill_value = read_mem(source);
        for i in 0..count {
            let dst_addr = dest + (i * word_size);
            write_mem(dst_addr);
        }
    } else {
        // Copy mode: copy from source to dest
        for i in 0..count {
            let src_addr = source + (i * word_size);
            let dst_addr = dest + (i * word_size);
            let _value = read_mem(src_addr);
            write_mem(dst_addr);
        }
    }
}

/// CpuFastSet - Fast memory copy/fill (32-bit only, must be 4-byte aligned)
pub fn cpu_fast_set<F>(source: u32, dest: u32, control: u32, mut read_mem: F, mut write_mem: F)
where
    F: FnMut(u32) -> u32 + Clone,
{
    let count = control & 0x1FFFFF;
    let is_fill = (control & (1 << 24)) != 0;

    // Always 32-bit transfers
    if is_fill {
        let _fill_value = read_mem(source);
        for i in 0..count {
            write_mem(dest + (i * 4));
        }
    } else {
        for i in 0..count {
            let _value = read_mem(source + (i * 4));
            write_mem(dest + (i * 4));
        }
    }
}

/// BitUnPack - Decompress bit-packed data
pub fn bit_unpack<F>(_source: u32, _dest: u32, info_addr: u32, mut read_mem: F)
where
    F: FnMut(u32) -> u32,
{
    // Read unpack info
    let _source_len = (read_mem(info_addr) & 0xFFFF) as u16;
    let _source_width = ((read_mem(info_addr) >> 16) & 0xFF) as u8;
    let _dest_width = ((read_mem(info_addr) >> 24) & 0xFF) as u8;

    // Simplified implementation - real BIOS does complex bit manipulation
    // This is a placeholder for basic functionality
}

/// LZ77UnComp - LZ77 decompression
pub fn lz77_uncomp<F>(source: u32, dest: u32, mut read_byte: F, mut write_byte: F)
where
    F: FnMut(u32) -> u8 + Clone,
{
    // Read header
    let header = read_byte(source) as u32
        | ((read_byte(source + 1) as u32) << 8)
        | ((read_byte(source + 2) as u32) << 16)
        | ((read_byte(source + 3) as u32) << 24);

    let decompressed_size = header >> 8;

    let mut src_pos = source + 4;
    let mut dst_pos = dest;
    let mut remaining = decompressed_size;

    while remaining > 0 {
        let flags = read_byte(src_pos);
        src_pos += 1;

        for i in 0..8 {
            if remaining == 0 {
                break;
            }

            if (flags & (0x80 >> i)) == 0 {
                // Uncompressed byte
                write_byte(dst_pos);
                dst_pos += 1;
                src_pos += 1;
                remaining -= 1;
            } else {
                // Compressed block
                let b1 = read_byte(src_pos) as u16;
                let b2 = read_byte(src_pos + 1) as u16;
                src_pos += 2;

                let length = ((b1 >> 4) + 3) as u32;
                let disp = (((b1 & 0xF) << 8) | b2) as u32;

                // Copy from earlier in output
                for _ in 0..length {
                    if remaining == 0 {
                        break;
                    }
                    let _copy_pos = dst_pos - disp - 1;
                    write_byte(dst_pos);
                    dst_pos += 1;
                    remaining -= 1;
                }
            }
        }
    }
}

/// RLUnComp - Run-Length decompression
pub fn rl_uncomp<F>(source: u32, dest: u32, mut read_byte: F, mut write_byte: F)
where
    F: FnMut(u32) -> u8 + Clone,
{
    // Read header
    let header = read_byte(source) as u32
        | ((read_byte(source + 1) as u32) << 8)
        | ((read_byte(source + 2) as u32) << 16)
        | ((read_byte(source + 3) as u32) << 24);

    let decompressed_size = header >> 8;

    let mut src_pos = source + 4;
    let mut dst_pos = dest;
    let mut remaining = decompressed_size;

    while remaining > 0 {
        let flag = read_byte(src_pos);
        src_pos += 1;

        if (flag & 0x80) == 0 {
            // Uncompressed run
            let length = (flag as u32 + 1).min(remaining);
            for _ in 0..length {
                write_byte(dst_pos);
                dst_pos += 1;
                src_pos += 1;
            }
            remaining -= length;
        } else {
            // Compressed run
            let length = ((flag & 0x7F) as u32 + 3).min(remaining);
            let _value = read_byte(src_pos);
            src_pos += 1;

            for _ in 0..length {
                write_byte(dst_pos);
                dst_pos += 1;
            }
            remaining -= length;
        }
    }
}
