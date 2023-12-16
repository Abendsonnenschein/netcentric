fn calc_chksm(data: &[u8]) -> u16 {
    let sum: u32 = data
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .fold(0, |acc, x| acc.wrapping_add(u32::from(x)));

    !((sum >> 16) + (sum & 0xFFFF)) as u16
}

#[rustfmt::skip]
fn main() {
    let data: [u8; 10] = [
        'N' as u8,
        'e' as u8,
        't' as u8,
        'w' as u8,
        'o' as u8,
        'r' as u8,
        'k' as u8,
        'i' as u8,
        'n' as u8,
        'g' as u8,
    ];

    let checksum: u16 = calc_chksm(&data);
    println!("Internet Checksum: {:b}", checksum);
}
