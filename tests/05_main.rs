use assert_cmd::Command;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_output() {
        let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();

        cmd.assert().success().stdout("Hello, world!\n");
    }

    #[test]
    fn cli_output_fail() {
        let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();

        cmd.assert().success().stdout("Failed Hello, world!\n");
    }
}

// cargo test --test 05_main