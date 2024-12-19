#![allow(dead_code)]

mod dunh;

fn main() {
  let code = "functon is_zero (x) {
    if (x == 0) [
      return true;
    ] else {
      return false;
    }
  }";
  println!("Error:");
  println!("{}", dunh::high_err_ctx(38, 64, code, 2));
  println!();

  println!("Warning:");
  println!("{}", dunh::high_warn(38, 64, code));
  println!();

  println!("Custom color:");
  println!("{}", dunh::high(38, 64, code, "\x1b[4m\x1b[32m"));
  println!();

  let code = "(Foo x) = 7[0 ]\n";
  println!("Warning:");
  println!("{}", dunh::high_err(16, 17, code));
  println!();
}
