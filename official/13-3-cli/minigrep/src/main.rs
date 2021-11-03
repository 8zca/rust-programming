extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // env::args() はiteratorを返すのでそのまま渡す
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 引数解析時に問題
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 所有権渡す
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}