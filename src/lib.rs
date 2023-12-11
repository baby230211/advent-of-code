use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("no file found"),
        };
        Ok(Config { file_path })
    }
}

pub fn run() -> Result<String, Box<dyn Error>> {
    let config = Config::new(env::args()).expect("lack of config");
    let contents = fs::read_to_string(config.file_path)?;
    return Ok(contents);
}
