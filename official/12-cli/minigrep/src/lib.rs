use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // 'static は静的ライフタイム. プログラム中ずっと生きてる
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // CLIだとpanic!ではなくResutを返す方がシンプル
            return Err("not enough arguments");
        }
    
        // 所有権問題の解消のためにcloneしても可
        // let query = args[1].clone();
        let query = &args[1];
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query: query.to_string(), filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // エラーが発生したらResultが返る
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    // エラーが発生したらResultが返る
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // Okを返す(値は無い)
    Ok(())
}

// 返却されるデータはcontentsと同じ期間だけ存在する
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
