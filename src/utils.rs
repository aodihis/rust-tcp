pub fn sum_bytes_as_u32(bytes: &[u8]) -> u32 {
    bytes.chunks(2)
        .map(|chunk| match chunk {
            [high, low] => u16::from_be_bytes([*high, *low]) as u32,
            [high] => u16::from_be_bytes([*high, 0]) as u32,
            _ => 0,
        })
        .sum()
}