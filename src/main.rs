use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // 命令行格式如右：cargo run -- 待搜索字符串 文件路径
    let args: Vec<String> = env::args().collect(); // env::args 读取到的参数中第一个就是程序的可执行路径名。
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        println!("Application Error: {e}");
        process::exit(1);
    }
}

