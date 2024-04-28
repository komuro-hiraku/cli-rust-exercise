use std::error::Error;

use clap::App;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
    .version("0.1.0")
    .author("komuro-hiraku <komuro.hiraku@gmail.com>")
    .about("Rust head")
    .get_matches();

    Ok(Config {
        files: todo!(),
        lines: todo!(),
        bytes: todo!(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    unimplemented!();
}