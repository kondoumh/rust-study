use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_config() {
        let args = [String::from("program"), String::from("word"), String::from("filename")];
        let config = Config::new(&args[..]).unwrap();
        assert_eq!(config.query, "word");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn config_param_error() {
        let args = [String::from("program"), String::from("word")];
        match Config::new(&args[..]) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(true);
                assert!(e.contains("not enough"));
            }
        }
    }
}