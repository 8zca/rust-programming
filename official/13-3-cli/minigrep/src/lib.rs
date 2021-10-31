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
    // argsの所有権を奪う
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // 最初は自身なのでnext
        args.next();

        // 検索文字列の取得
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        // ファイル名の取得
        let filename = match args.next() {
            Some(arg) => arg,
            // ファイル名を取得しませんでした
            None => return Err("Didn't get a file name"),
        };

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
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // forループよりイテレータのほうが速い
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
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
