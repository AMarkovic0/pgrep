mod config_file;

use std::env;
use std::process::Command;
use std::path::Path;
use std::rc::Rc;

use crate::cache_handler::config_file::FilePath;
use crate::cache_handler::config_file::ConfigFile;

pub struct Cache {
    path:  FilePath,
    config: ConfigFile,
    history: ConfigFile
}

impl Cache {
    pub fn new() -> Option<Cache> {
        if let Some(path) = env::home_dir() {
            let path = Rc::new(format!("{}/{}", path.display(), ".pgrep"));

            if !Path::new(&(*path)).exists() {
                Command::new("mkdir")
                    .arg(&(*path))
                    .spawn()
                    .expect("ERROR: Failed to wpawn mkdir")
                    .wait()
                    .expect("ERROR: mkdir execution failed");
            }

            return Some(Cache {
                path: FilePath::Referenced(Rc::clone(&path)),
                config: ConfigFile::new(
                    FilePath::Referenced(Rc::clone(&path)),
                    "config.toml".to_string()
                ).expect("ERROR: Failed to create config file"),
                history: ConfigFile::new(
                    FilePath::Referenced(Rc::clone(&path)),
                    "history.log".to_string()
                ).expect("ERROR: Failed to create history file")
            })
        }

        None
    }

    pub fn cache_history(&self, args: &String) -> std::io::Result<()> {
        self.history.write_line(args)?;
        Ok(())
    }

    pub fn read_history(&self) -> std::io::Result<String> {
        self.history.read()
    }

    pub fn print_history(&self) -> std::io::Result<()> {
        self.history.print()?;
        Ok(())
    }
}
