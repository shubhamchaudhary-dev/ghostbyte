use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::checksum::cal;
use crate::footer::{Ftr, FTS, MAG};
pub fn run(hos: &str) -> Result<(), String> {
    let mut fil = File::open(format!("samples/{hos}"))
        .or_else(|_| File::open(hos))
        .map_err(|e| e.to_string())?;
    let siz = fil
        .metadata()
        .map_err(|e| e.to_string())?
        .len();
    let mpo = siz - MAG.len() as u64;
    fil.seek(SeekFrom::Start(mpo))
        .map_err(|e| e.to_string())?;
    let mut mag = vec![0; MAG.len()];
    fil.read_exact(&mut mag)
        .map_err(|e| e.to_string())?;
    if mag != MAG {
        return Err("GhostByte payload not found".into());
    }
    let fpo = mpo - FTS as u64;
    fil.seek(SeekFrom::Start(fpo))
        .map_err(|e| e.to_string())?;
    let mut byt = vec![0; FTS];
    fil.read_exact(&mut byt)
        .map_err(|e| e.to_string())?;
    let ftr = Ftr::frb(&byt)?;
    let npo = fpo - ftr.nln as u64;
    fil.seek(SeekFrom::Start(npo))
        .map_err(|e| e.to_string())?;
    let mut nam = vec![0; ftr.nln as usize];
    fil.read_exact(&mut nam)
        .map_err(|e| e.to_string())?;
    let nam = String::from_utf8(nam)
        .map_err(|_| "Invalid filename")?;
    fil.seek(SeekFrom::Start(ftr.off))
        .map_err(|e| e.to_string())?;
    let mut pay = vec![0; ftr.siz as usize];
    fil.read_exact(&mut pay)
        .map_err(|e| e.to_string())?;
    if cal(&pay) != ftr.sum {
        return Err("Checksum mismatch".into());
    }
    File::create(&nam)
        .and_then(|mut out| out.write_all(&pay))
        .map_err(|e| e.to_string())?;
    println!(
        "\n[GhostByte Extraction]
Recovered : {}
Method    : Reverse Footer Parsing
Status    : Success\n",
        nam
    );
    Ok(())
}