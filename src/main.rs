mod code_grep_results;
mod cache_handler;
mod cli;

use std::env;
use std::error::Error;
use std::process::Command;
use std::fs;
use std::path::PathBuf;

use crate::code_grep_results::GrepRes;
use crate::cache_handler::Cache;

fn open_vim(selected_element: Option<&GrepRes>) {
    if let Some(selected) = selected_element {
        Command::new("vim")
            .arg(format!("+{}", selected.getl()))
            .arg(fs::canonicalize(&PathBuf::from(selected.getp()))
                .expect("ERROR: Selected path does not exists"))
            .spawn()
            .expect("ERROR: Vim opening failed")
            .wait()
            .expect("ERROR: Vim execution failed");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let home_dir_path = env::home_dir().expect("ERROR: Failed getting path to home dir");
    if let Some(c) = Cache::new(home_dir_path) {
        c.cache_history(&args.join(" "))?;
        if cli::check_for_history() {
            c.print_history()?;
            return Ok(())
        }
    };

    if cli::check_for_help() {
        cli::print_help();
        return Ok(())
    }

    let res = Command::new("grep")
        .args(args)
        .arg("-rn")
        .output()
        .expect("Error: grep command failed to execute");

    let res = String::from_utf8(res.stdout)
        .expect("ERROR: Cannot convert grep output to string");
    let res_vec = GrepRes::deserialize_output(res);

    if res_vec.len() > 0 {
        open_vim(res_vec.get(cli::select_output()));
    }

    Ok(())
}
