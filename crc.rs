fn crc(d: u64, gen: u64) -> u64 {
    let gsz: usize = (g as f64).log2() as usize + 1;
    let mut div: u64 = d << (gsz - 1);
    let mut shft: usize = 0;

    while div >= gen || shft > 0 {
        let d_next = (div + 1).next_power_of_two().trailing_zeros() as usize;
        shft = std::cmp::max(d_next, gsz) - gsz;
        let rem: u64 = (div >> shft) ^ gen;
        div = (div & ((1 << shft) - 1)) | (rem << shft);
    }

    div
}

fn main() {
    let d: u64 = 0b1010101010;
    let g: u64 = 0b10011;
    let r: u64 = crc(d, g);

    println!("R: {:b}", r);
}
