use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let  file=OpenOptions::new().read(true).write(true).create(true).open(path);
   let _= file.unwrap().write(content.as_bytes());
}