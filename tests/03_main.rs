use assert_cmd::Command;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn cli_test(){
        let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();
        cmd.assert().success();

    }

    #[test]
    fn cli_output(){
        //let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();
        //cmd.assert().success();

        //let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();
        
        //let assert = cmd
        //.arg("-A")
        //.env("stdout", "hello")
        //.env("exit", "42")
        //.write_stdin("42")
        //.assert();
        //assert.failure().code(42).stdout("Hello, world!\n");

        let mut cmd = Command::cargo_bin("rust_main_startup").unwrap();
        // cmd.env_clear().env("stdout", "Hello, world!\n");
        cmd.assert().success().stdout("Hello, world!\n");
        //assert().success().stdout("42\n");

    }
}

