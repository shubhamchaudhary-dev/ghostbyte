use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use crate::checksum::cal;
use crate::footer::{Ftr, MAG};
pub fn run(pay: &str, hos: &str) -> Result<(), String> {
    let mut dat = Vec::new();
    File::open(format!("samples/{pay}"))
        .or_else(|_| File::open(pay))
        .and_then(|mut fil| fil.read_to_end(&mut dat))
        .map_err(|e| e.to_string())?;
    let mut fil = OpenOptions::new()
        .append(true)
        .read(true)
        .open(format!("samples/{hos}"))
        .or_else(|_| {
            OpenOptions::new()
                .append(true)
                .read(true)
                .open(hos)
        })
        .map_err(|e| e.to_string())?;
    let off = fil
        .seek(SeekFrom::End(0))
        .map_err(|e| e.to_string())?;
    let ftr = Ftr {
        ver: 1,
        siz: dat.len() as u64,
        sum: cal(&dat),
        nln: pay.len() as u16,
        off,
        flg: 0,
    };
    for chk in [
        &dat[..],
        pay.as_bytes(),
        &ftr.tob(),
        MAG,
    ] {
        fil.write_all(chk)
            .map_err(|e| e.to_string())?;
    }
    println!(
        "\n[GhostByte]

Payload  : {}
Host     : {}
Status   : Implanted
Mode     : Append-Only\n",
        pay,
        hos
    );
    Ok(())
}