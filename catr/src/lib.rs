use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
  println!("Hello world!");
  Ok(())
}

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_nonblank_lines: bool,
}

fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
    .version("0.1.0")
    .author("tatenodev <@tatenodev>")
    .about("Rust cat")
    .arg(
      Arg::with_name("files")
        .value_name("FILES")
        .help("Input file path")
        .required(true)
        .min_values(1)
    )
    .arg(
      Arg::with_name("number_lines")
         .short("n")
         .help("foo")
         .takes_value(false)
    )
    .arg(
      Arg::with_name("number_nonblank_lines")
      .short("b")
      .help("bar")
      .takes_value(false)
    ).get_matches();

  let files = matches.values_of_lossy("files").unwrap();
  let number_lines = matches.is_present("number_lines");
  let number_nonblank_lines = matches.is_present("number_nonblank_lines");

  Ok(Config {
    files: files,
    number_lines: number_lines,
    number_nonblank_lines: number_nonblank_lines,
  })
}
