use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else( |err | {
        eprintln!("인수를 구문분석하는 동안 오류가 발생하였습니다. {}", err);
        process::exit(1);
    });

    println!("검색어: {}", config.query);
    println!("파일명: {}", config.filename);

    if let Err(e) = minigrep::run(config){
        eprintln!("어플리케이션 에러 {}", e);
        process::exit(1)
    }
}

