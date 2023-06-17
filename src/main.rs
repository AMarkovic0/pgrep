use std::env;
use std::error::Error;
use std::process::Command;
use std::fs;
use std::path::PathBuf;

use code_grep_results::GrepRes;

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

mod code_grep_results {
    use std::str::FromStr;

    use regex::Regex;

    #[derive(Debug)]
    pub struct GrepRes {
        path: String,
        line: u32,
        text: String,
    }

    impl GrepRes {
        pub fn new(s: &str) -> Option<GrepRes> {
            match Regex::new(r"(.*):(\d+)(.*)").ok()?.captures(s) {
                Some(captures) => Some(GrepRes {
                    path: captures[1].to_string(),
                    line: FromStr::from_str(&captures[2]).unwrap(),
                    text: captures[3].to_string(),
                }),
                None => {
                    None
                }
            }
        }

        pub fn getl(&self) -> u32 {
            self.line
        }

        pub fn getp(&self) -> &String {
            &self.path
        }

        pub fn print(&self, index: usize) {
            println!("[{:?}] File: {:?} Line: {:?} Text: {:?}", index, self.path, self.line, self.text);
        }
    }
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
