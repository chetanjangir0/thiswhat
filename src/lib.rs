use std::env;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let path = match args.next() {
            Some(p) => p,
            None => return Err("please specify the path"),
        };

        Ok(Self { path })
    }
}

