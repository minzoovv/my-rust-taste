use std::env;
use std::fs;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}
        

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let lines = if config.case_sensitive {
        search(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            lines.push(line);
        }
    }
    lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    let query = query.to_lowercase();
    let mut lines = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            lines.push(line);
        }
    }
    lines

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
rust nice! 
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
rust nice! 
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )   
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
rust nice!
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["rust nice!"], search_case_insensitive(query, contents)
        );
    }
}