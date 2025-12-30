#![allow(dead_code, unused_imports, unused_variables)]

//REM: This is the 'binary crate and/or binary module' or also known as 'main-entry-point' (bin|executable).
fn main() {
  println!("{} {}", "Hi", "there!");
}

//REM: This is a 'unit-test'
#[cfg(test)]
#[test]
fn unit_test_something_with_main() {
  //REM: --- something ---
}

#[test]
fn unit_test_something_with_main_i() {
  //REM: --- something ---
}

#[test]
fn unit_test_something_with_main_ii() {
  //REM: --- something ---
}
