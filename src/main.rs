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
        std::io::stdin().read_line(&mut inner).expect("ERROR: String expected.");
        let $out = inner.trim().parse::<$type>().expect("ERROR: Parsing user input.");
    };
}

fn deserialize_output(res: String) -> Vec<GrepRes> {
    let mut res_vec = Vec::new();
    let mut index = 0;

    for r in res.split("\n").collect::<Vec<&str>>() {
        if let Some(gres) = GrepRes::new(r) {
            gres.print(index);
            res_vec.push(gres);
            index += 1;
        }
    }

    res_vec
}

fn select_output() -> usize {
    read!(selected as usize);
    selected
}

fn open_vim(selected_element: Option<&GrepRes>) {
    if let Some(selected) = selected_element {
        Command::new("vim")
            .arg(format!("+{}", selected.getl()))
            .arg(fs::canonicalize(&PathBuf::from(selected.getp()))
                .expect("ERROR: Selected path does not exists."))
            .spawn()
            .expect("ERROR: Vim opening failed.")
            .wait()
            .expect("ERROR: Vim execution failed.");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let res = Command::new("grep")
        .args(env::args().skip(1))
        .arg("-rn")
        .output()
        .expect("Error: grep command failed to execute.");

    let res = String::from_utf8(res.stdout).expect("ERROR: Cannot convert grep output to string.");
    let res_vec = deserialize_output(res);

    if res_vec.len() > 0 {
        open_vim(res_vec.get(select_output()));
    }

    Ok(())
}
