pub fn cal(dat: &[u8]) -> u32 {
    dat.iter()
        .fold(0u32, |acc, &byt| {
            acc.wrapping_add(byt as u32)
        })
}