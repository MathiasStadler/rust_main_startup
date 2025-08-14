use assert_cmd::Command;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_test() {
        let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();
        cmd.assert().success();
    }

    #[test]
    fn cli_output() {
        let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();

        cmd.assert().success().stdout("Hello, world!\n");
    }
}
