use std::env;
pub enum Cmd {
    Imp(String, String),
    Ext(String),
    Ins(String),
}
pub fn prs() -> Result<Cmd, String> {
    let arg: Vec<String> = env::args().collect();
    match arg.get(1).map(|s| s.as_str()) {
        Some("implant") if arg.len() == 4 =>
            Ok(Cmd::Imp(
                arg[2].clone(),
                arg[3].clone(),
            )),
        Some("extract") if arg.len() == 3 =>
            Ok(Cmd::Ext(arg[2].clone())),
        Some("inspect") if arg.len() == 3 =>
            Ok(Cmd::Ins(arg[2].clone())),
        _ => Err(
            "Usage:
ghostbyte implant <payload> <host>
ghostbyte extract <host>
ghostbyte inspect <host>"
                .into(),
        ),
    }
}