// test case for main.rs
// found binary in path ./debug/rust_main_startup
use assert_cmd::Command;

let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();
cmd.assert().success();

