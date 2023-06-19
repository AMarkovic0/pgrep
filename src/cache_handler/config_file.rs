use std::rc::Rc;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub enum FilePath {
    Raw(String),
    Referenced(Rc<String>)
}

impl FilePath {
    fn format_filename(&self, fname: &str) -> String {
        match self {
            FilePath::Raw(s) => format!("{}/{}", s, fname),
            FilePath::Referenced(r) => format!("{}/{}", *r, fname)
        }
    }
}

pub struct ConfigFile {
    path: FilePath,
    name: String
}

impl ConfigFile {
    pub fn new(fpath: FilePath, fname: String) -> std::io::Result<ConfigFile> {
        let full_path = fpath.format_filename(&fname);

        if !Path::new(&full_path).exists() {
            File::create(&full_path)?;
        }

        Ok(ConfigFile {
            path: fpath,
            name: fname
        })
    }

    pub fn read(&self) -> std::io::Result<String> {
        let full_path = self.path.format_filename(&self.name);
        let mut contents = String::new();

        File::open(&full_path)?
            .read_to_string(&mut contents)?;

        Ok(contents)
    }

    pub fn write_line(&self, line: &String) -> std::io::Result<()> {
        let full_path = self.path.format_filename(&self.name);

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(full_path)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", &line) {
            return Err(e)
        }

        Ok(())
    }

    pub fn print(&self) -> std::io::Result<()> {
        println!("{}", self.read()?);
        Ok(())
    }
}
