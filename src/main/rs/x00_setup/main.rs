#![allow(dead_code, unused_imports, unused_variables)]

//REM: This is the 'main module' or 'main-entry-point' of this crate.
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
