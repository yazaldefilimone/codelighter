#![allow(dead_code)]
mod codelighter;

fn main() {
  let code = "functon is_zero (x) {
    if (x == 0) [
      return true;
    ] else {
      return false;
    }
  }";
  println!("Error:");
  println!("{}", codelighter::highlight_error(38, 64, code));
  println!();

  println!("Warning:");
  println!("{}", codelighter::highlight_warn(38, 64, code));
  println!();

  println!("Custom color:");
  println!("{}", codelighter::highlight(38, 64, code, "\x1b[4m\x1b[32m", 0));
  println!();

  let code = "(Foo x) = 7[0 ]\n";
  println!("Warning:");
  println!("{}", codelighter::highlight_error(16, 17, code));
  println!();
}
