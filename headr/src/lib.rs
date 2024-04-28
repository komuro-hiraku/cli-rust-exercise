use std::error::Error;
use clap::{App, Arg};

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

    // Flag: -h, -V
    // Options: -c, -n
    // Args: FILE...
    .arg(
        Arg::with_name("number_of_bytes")
    )
    .arg(
        Arg::with_name("number_of_lines")
    )
    .get_matches();

    // TODO: lines, bytes をそれぞれ数値化するとか色々

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
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),

        // Err(val.into()), Err(Into::into(val)) でも良いらしい
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3は正の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 数字でない文字列の場合はエラー
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}