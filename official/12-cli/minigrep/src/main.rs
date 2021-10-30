extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // コマンドライン引数がargsに入る
    // $ cargo run foo bar
    // ["minigrep", "foo", "bar"]
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
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
