use rand::Rng;
use std::process::exit;

#[derive(Debug)]
pub struct Config {
  uppercase: bool,
  numbers: bool,
  symbols: bool,
  length: u32,
}

fn print_help() {
  println!("pagen - simple password generator made in Rust");
  println!();
  println!("USAGE: pagen [OPTIONS]");
  println!();
  println!("OPTIONS:");
  println!("     -u         Include uppercase symbols");
  println!("     -n         Include numbers");
  println!("     -l LENGTH  Specify the password length, default is 8");
  println!("     -s         Include symbols");
  println!("     -h, --help Show this help message");
}

pub fn parse_args(args: Vec<String>) -> Result<Config, String> {
  let mut output = Config {
    uppercase: false,
    numbers: false,
    symbols: false,
    length: 8,
  };

  let mut args_iter = args.into_iter().skip(1).peekable();
  while let Some(arg) = args_iter.next() {
    match arg.as_str() {
      "-h" | "--help" => {
        print_help();
        exit(0);
      }
      "-u" => output.uppercase = true,
      "-n" => output.numbers = true,
      "-s" => output.symbols = true,
      "-l" => {
        if let Some(len_str) = args_iter.next() {
          if let Ok(len) = len_str.parse::<u32>() {
            output.length = len;
          }
        }
      }
      _ => {
        return Err(format!(
          "Invalid argument {} \nRun with --help for more information",
          arg
        ));
      }
    }
  }

  Ok(output)
}

pub fn generate_password(config: Config) -> String {
  let mut password = String::new();
  let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");
  if config.uppercase {
    charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
  }
  if config.numbers {
    charset.push_str("0123456789");
  }
  if config.symbols {
    charset.push_str("!@#$%^&*()-_+=|\\{}[]:;\"'<>,.?/");
  }
  let mut rng = rand::thread_rng();
  for _ in 0..config.length {
    let random_index = rng.gen_range(0..charset.len());
    password.push(charset.chars().nth(random_index).unwrap());
  }
  password
}
