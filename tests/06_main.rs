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
    #[should_panic]
    fn cli_output_fail_exact() {
        let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();

        cmd.assert().success().stdout("Failed Hello, world!\n");
    }
}

// cargo test --test 05_main

// only testcase via cmd line 
// cargo test --test 05_main
// cargo test --test 05_main cli_output_fail
// cargo test tests::cli_output_fail  -- --exact
// cargo test -- --list
// cargo test tests::cli_output_fail_exact  -- --exact

// test coverage

