mod code_grep_results;

use std::env;
use std::error::Error;
use std::process::Command;
use std::fs;
use std::path::PathBuf;

use crate::code_grep_results::GrepRes;

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

fn open_vim(res_vec: Vec<GrepRes>) -> Result<(), Box<dyn Error>> {
    if res_vec.len() > 0 {
        read!(selected as usize);
        if let Some(selected) = res_vec.get(selected) {
            Command::new("vim")
                .arg(format!("+{}", selected.getl()))
                .arg(fs::canonicalize(&PathBuf::from(selected.getp()))?)
                .spawn()
                .expect("ERROR: Vim opening failed.")
                .wait()
                .expect("ERROR: Vim execution failed.");
        }

        return Ok(())
    }

    Err("No results found.".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut grep_cmd = Command::new("grep");

    let s = grep_cmd
        .args(env::args().skip(1))
        .arg("-rin")
        .output()
        .expect("Error: grep command failed to execute.");

    let res = String::from_utf8(s.stdout).expect("ERROR: Cannot covert grep output to string.");
    let mut res_vec = Vec::new();

    let mut index = 0;
    for r in res.split("\n").collect::<Vec<&str>>() {
        if let Some(gres) = GrepRes::new(r) {
            gres.print(index);
            res_vec.push(gres);
            index += 1;
        }
    }

    open_vim(res_vec)?;

    Ok(())
}
