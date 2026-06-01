pub const MAG: &[u8] = b"GHOSTBYTEv1";
pub const FTS: usize = 24;
pub struct Ftr {
    pub ver: u8,
    pub siz: u64,
    pub sum: u32,
    pub nln: u16,
    pub off: u64,
    pub flg: u8,
}
impl Ftr {
    pub fn tob(&self) -> Vec<u8> {
        [
            &[self.ver][..],
            &self.siz.to_le_bytes(),
            &self.sum.to_le_bytes(),
            &self.nln.to_le_bytes(),
            &self.off.to_le_bytes(),
            &[self.flg],
        ]
        .concat()
    }
    pub fn frb(byt: &[u8]) -> Result<Self, String> {
        if byt.len() != FTS {
            return Err("Bad footer".into());
        }
        Ok(Self {
            ver: byt[0],
            siz: u64::from_le_bytes(
                byt[1..9].try_into().unwrap()
            ),
            sum: u32::from_le_bytes(
                byt[9..13].try_into().unwrap()
            ),
            nln: u16::from_le_bytes(
                byt[13..15].try_into().unwrap()
            ),
            off: u64::from_le_bytes(
                byt[15..23].try_into().unwrap()
            ),
            flg: byt[23],
        })
    }
}