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
        match Regex::new(r"(.*):(\d+):\s+(.*)").ok()?.captures(s) {
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
        println!("[{}] File: {} Line: {} Text: {}", index, self.path, self.line, self.text);
    }
}
