mod utils;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  match utils::parse_args(args) {
    Ok(config) => {
      println!("{}", utils::generate_password(config));
    }
    Err(e) => {
      eprintln!("{}", e);
    }
  }
}
